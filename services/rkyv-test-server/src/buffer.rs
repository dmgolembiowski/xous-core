use rkyv::{ser::Serializer, Fallible};
use xous::{
    map_memory, send_message, unmap_memory, Error, MemoryAddress, MemoryFlags, MemoryMessage,
    MemoryRange, MemorySize, Message, Result, CID,
};

#[derive(Debug)]
pub struct Buffer<'a> {
    range: MemoryRange,
    valid: MemoryRange,
    offset: Option<MemoryAddress>,
    slice: &'a mut [u8],
    should_drop: bool,
}

pub struct XousDeserializer;

// Unreachable enum pattern, swap out for the never type (!) whenever that gets stabilized
#[derive(Debug)]
pub enum XousUnreachable {}

impl rkyv::Fallible for XousDeserializer {
    type Error = XousUnreachable;
}

impl<'a> Buffer<'a> {
    #[allow(dead_code)]
    pub fn new(len: usize) -> Self {
        let remainder = if ((len & 0xFFF) == 0) && (len > 0) {
            0
        } else {
            0x1000 - (len & 0xFFF)
        };

        let flags = MemoryFlags::R | MemoryFlags::W;

        // Allocate enough memory to hold the requested data
        let new_mem = map_memory(
            None,
            None,
            // Ensure our byte size is a multiple of 4096
            len + remainder,
            flags,
        )
        .expect("Buffer: error in new()/map_memory");

        let mut valid = new_mem;
        valid.size = MemorySize::new(len + remainder).unwrap();
        Buffer {
            range: new_mem,
            slice: unsafe {
                core::slice::from_raw_parts_mut(new_mem.as_mut_ptr(), len + remainder)
            },
            valid,
            offset: None,
            should_drop: true,
        }
    }

    #[allow(dead_code)]
    pub unsafe fn from_memory_message(mem: &'a MemoryMessage) -> Self {
        Buffer {
            range: mem.buf,
            slice: core::slice::from_raw_parts_mut(mem.buf.as_mut_ptr(), mem.buf.len()),
            valid: mem.buf,
            offset: mem.offset,
            should_drop: false,
        }
    }

    /// Perform a mutable lend of this Buffer to the server.
    #[allow(dead_code)]
    pub fn lend_mut(&mut self, connection: CID, id: u32) -> core::result::Result<Result, Error> {
        let msg = MemoryMessage {
            id: id as usize,
            buf: self.valid,
            offset: self.offset,
            valid: MemorySize::new(self.slice.len()),
        };

        // Update the offset pointer if the server modified it.
        let result = send_message(connection, Message::MutableBorrow(msg));
        if let Ok(Result::MemoryReturned(offset, _valid)) = result {
            self.offset = offset;
        }

        result
    }

    #[allow(dead_code)]
    pub fn lend(&self, connection: CID, id: u32) -> core::result::Result<Result, Error> {
        let msg = MemoryMessage {
            id: id as usize,
            buf: self.valid,
            offset: self.offset,
            valid: MemorySize::new(self.slice.len()),
        };
        send_message(connection, Message::Borrow(msg))
    }

    #[allow(dead_code)]
    pub fn send(mut self, connection: CID, id: u32) -> core::result::Result<Result, Error> {
        let msg = MemoryMessage {
            id: id as usize,
            buf: self.valid,
            offset: self.offset,
            valid: MemorySize::new(self.slice.len()),
        };
        let result = send_message(connection, Message::Move(msg))?;

        // prevents it from being Dropped.
        self.should_drop = false;
        Ok(result)
    }

    #[allow(dead_code)]
    pub fn try_from<S>(src: S) -> core::result::Result<Self, ()>
    where
        S: rkyv::Serialize<rkyv::ser::serializers::BufferSerializer<Buffer<'a>>>,
    {
        let buf = Self::new(4096);
        let mut ser = rkyv::ser::serializers::BufferSerializer::new(buf);
        let pos = ser.serialize_value(&src).or(Err(()))?;
        let mut buf = ser.into_inner();
        buf.offset = MemoryAddress::new(pos);
        Ok(buf)
    }

    #[allow(dead_code)]
    pub fn serialize_from<S>(self, src: S) -> core::result::Result<Self, ()>
    where
        S: rkyv::Serialize<rkyv::ser::serializers::BufferSerializer<Buffer<'a>>>,
    {
        let mut ser = rkyv::ser::serializers::BufferSerializer::new(self);
        let pos = ser.serialize_value(&src).or(Err(()))?;
        let mut buf = ser.into_inner();
        buf.offset = MemoryAddress::new(pos);
        Ok(buf)
    }

    #[allow(dead_code)]
    pub fn try_into<T, U>(&self) -> core::result::Result<&U, ()>
    where
        T: rkyv::Archive<Archived = U>,
    {
        let pos = self.offset.map(|o| o.get()).unwrap_or_default();
        let r = unsafe { rkyv::archived_value::<T>(self.slice, pos) };
        Ok(r)
    }

    #[allow(dead_code)]
    pub fn deserialize<T, U>(&self) -> core::result::Result<T, ()>
    where
        T: rkyv::Archive<Archived = U>,
        U: rkyv::Deserialize<T, dyn Fallible<Error = XousUnreachable>>,
    {
        let pos = self.offset.map(|o| o.get()).unwrap_or_default();
        let r = unsafe { rkyv::archived_value::<T>(self.slice, pos) };
        Ok(r.deserialize(&mut XousDeserializer {}).unwrap())
    }
}

impl<'a> core::convert::AsRef<[u8]> for Buffer<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}

impl<'a> core::convert::AsMut<[u8]> for Buffer<'a> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.slice
    }
}

impl<'a> core::ops::Deref for Buffer<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.slice
    }
}

impl<'a> core::ops::DerefMut for Buffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.slice
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {
        if self.should_drop {
            unmap_memory(self.range).expect("Buffer: failed to drop memory");
        }
    }
}
