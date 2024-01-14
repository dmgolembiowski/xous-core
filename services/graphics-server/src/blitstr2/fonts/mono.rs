#![rustfmt::skip]
// DO NOT MAKE EDITS HERE because this file is automatically generated.
// To make changes, see <xous_root>/services/graphics-server/src/blitstr2/codegen/main.go
//
// This code includes encoded bitmaps of glyphs from the Courier typeface which
// was designed by Howard G. "Bud" Kettler in the 1950's for use with IBM
// typewriters. The PNG sprite sheet of Courier glyphs came from a screenshot
// taken on Macintosh System 7, with additional Latin Extended characters added
// by Sam Blenny.
//
//! Mono Font
#![allow(dead_code)]

/// Maximum height of glyph patterns in this bitmap typeface.
pub const MAX_HEIGHT: u8 = 15;

/// Unicode character codepoints corresponding to glyph sprites in GLYPHS array.
/// Indended use:
///  1. Do binary search on CODEPOINTS to find index of the codepoint corresponding
///     to the glyph you want to locate
///  2. Multiply resulting CODEPOINTS index by 8 (<<3) to get index into GLYPHS for
///     the corresponding glyph sprite (because 16*16px sprite size is 8*u32)
pub const CODEPOINTS: [u32; 207] = [
0x00020,
0x00021,
0x00022,
0x00023,
0x00024,
0x00025,
0x00026,
0x00027,
0x00028,
0x00029,
0x0002A,
0x0002B,
0x0002C,
0x0002D,
0x0002E,
0x0002F,
0x00030,
0x00031,
0x00032,
0x00033,
0x00034,
0x00035,
0x00036,
0x00037,
0x00038,
0x00039,
0x0003A,
0x0003B,
0x0003C,
0x0003D,
0x0003E,
0x0003F,
0x00040,
0x00041,
0x00042,
0x00043,
0x00044,
0x00045,
0x00046,
0x00047,
0x00048,
0x00049,
0x0004A,
0x0004B,
0x0004C,
0x0004D,
0x0004E,
0x0004F,
0x00050,
0x00051,
0x00052,
0x00053,
0x00054,
0x00055,
0x00056,
0x00057,
0x00058,
0x00059,
0x0005A,
0x0005B,
0x0005C,
0x0005D,
0x0005E,
0x0005F,
0x00060,
0x00061,
0x00062,
0x00063,
0x00064,
0x00065,
0x00066,
0x00067,
0x00068,
0x00069,
0x0006A,
0x0006B,
0x0006C,
0x0006D,
0x0006E,
0x0006F,
0x00070,
0x00071,
0x00072,
0x00073,
0x00074,
0x00075,
0x00076,
0x00077,
0x00078,
0x00079,
0x0007A,
0x0007B,
0x0007C,
0x0007D,
0x0007E,
0x000A0,
0x000A1,
0x000A2,
0x000A3,
0x000A4,
0x000A5,
0x000A6,
0x000A7,
0x000A8,
0x000A9,
0x000AA,
0x000AB,
0x000AC,
0x000AD,
0x000AE,
0x000AF,
0x000B0,
0x000B1,
0x000B2,
0x000B3,
0x000B4,
0x000B5,
0x000B6,
0x000B7,
0x000B8,
0x000B9,
0x000BA,
0x000BB,
0x000BC,
0x000BD,
0x000BE,
0x000BF,
0x000C0,
0x000C1,
0x000C2,
0x000C3,
0x000C4,
0x000C5,
0x000C6,
0x000C7,
0x000C8,
0x000C9,
0x000CA,
0x000CB,
0x000CC,
0x000CD,
0x000CE,
0x000CF,
0x000D0,
0x000D1,
0x000D2,
0x000D3,
0x000D4,
0x000D5,
0x000D6,
0x000D7,
0x000D8,
0x000D9,
0x000DA,
0x000DB,
0x000DC,
0x000DD,
0x000DE,
0x000DF,
0x000E0,
0x000E1,
0x000E2,
0x000E3,
0x000E4,
0x000E5,
0x000E6,
0x000E7,
0x000E8,
0x000E9,
0x000EA,
0x000EB,
0x000EC,
0x000ED,
0x000EE,
0x000EF,
0x000F0,
0x000F1,
0x000F2,
0x000F3,
0x000F4,
0x000F5,
0x000F6,
0x000F7,
0x000F8,
0x000F9,
0x000FA,
0x000FB,
0x000FC,
0x000FD,
0x000FE,
0x000FF,
0x00152,
0x00153,
0x02018,
0x02019,
0x0201A,
0x0201B,
0x0201C,
0x0201D,
0x0201E,
0x0201F,
0x02020,
0x02021,
0x02022,
0x02026,
0x020AC,
0x0FFFD,
];

#[cfg(any(feature="precursor", feature="renode"))]
pub(crate) static GLYPH_LOCATION: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) const GLYPH_LEN: usize = 1656;

pub(crate) fn glyphs() -> &'static [u32] {
    #[cfg(any(feature="precursor", feature="renode"))]
    unsafe {
        let data: *const u32 = core::mem::transmute(GLYPH_LOCATION.load(core::sync::atomic::Ordering::SeqCst));
        core::slice::from_raw_parts(data, GLYPH_LEN)
    }

    #[cfg(not(target_os = "xous"))]
    &GLYPHS
}

#[cfg(not(target_os = "xous"))]
/// Packed 16px * 16px glyph pattern data.
/// Pixels are packed in row-major order with LSB of first pixel word
/// containing the top left pixel. Bit of 0 means clear, 1 means set
pub const GLYPHS: [u32; 1656] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080008, 0x00080008, 0x00080008, 0x00080000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00140014, 0x00000014, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00140014, 0x0014003e, 0x0014003e, 0x00000014, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x002a001c, 0x001c000a, 0x00280028, 0x001c002a, 0x00000008, 0x00000000,
    0x00000000, 0x00000000, 0x00250022, 0x000a0015, 0x002a0014, 0x00110029, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00090006, 0x00020001, 0x00290005, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040004, 0x00000004, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080010, 0x00040008, 0x00040004, 0x00080004, 0x00100008, 0x00000000,
    0x00000000, 0x00000000, 0x00080004, 0x00100008, 0x00100010, 0x00080010, 0x00040008, 0x00000000,
    0x00000000, 0x00000000, 0x002a0008, 0x001c001c, 0x0008002a, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00080008, 0x0008003e, 0x00000008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00080000, 0x00000004, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x0000003e, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00040000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00100010, 0x00080008, 0x00040004, 0x00020002, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00220022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x000c0008, 0x0008000a, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00100020, 0x00040008, 0x003e0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00180020, 0x00200020, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00180010, 0x00120014, 0x0010003e, 0x00380010, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0002003e, 0x001e0002, 0x00200022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040018, 0x001e0002, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003e, 0x00100020, 0x00080010, 0x00080008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x001c0022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00220022, 0x0020003c, 0x000c0010, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000008, 0x00000000, 0x00080000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000008, 0x00000000, 0x00080000, 0x00000004, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x000c0030, 0x000c0003, 0x00000030, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x003f0000, 0x003f0000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x000c0003, 0x000c0030, 0x00000003, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00200022, 0x00080010, 0x00080000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00250039, 0x00250025, 0x00020039, 0x0000003c, 0x00000000,
    0x00000000, 0x00000000, 0x0008000c, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001f, 0x001e0022, 0x00220022, 0x001f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003c, 0x00010001, 0x00010001, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012000f, 0x00220022, 0x00220022, 0x000f0012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003f, 0x000e000a, 0x0002000a, 0x003f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003f, 0x000e000a, 0x0002000a, 0x000f0002, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003c, 0x00010001, 0x00210071, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x003e0022, 0x00220022, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0008003e, 0x00080008, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0010003c, 0x00100010, 0x00110010, 0x000e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00120037, 0x0006000a, 0x000a0006, 0x00370012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00020007, 0x00020002, 0x00220002, 0x003f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00360063, 0x0022002a, 0x00220022, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00260073, 0x002a0026, 0x0032002a, 0x00270032, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001f, 0x00220022, 0x0002001e, 0x000f0002, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000032, 0x00000000,
    0x00000000, 0x00000000, 0x0022001f, 0x00220022, 0x000a001e, 0x00270012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003c, 0x001c0002, 0x00200020, 0x001e0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0049007f, 0x00080049, 0x00080008, 0x001c0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x00220022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x00140022, 0x00080014, 0x00080008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x002a002a, 0x0014002a, 0x00140014, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x00080014, 0x00140008, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x00080014, 0x00080008, 0x001c0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0021003f, 0x00080011, 0x00220004, 0x003f0021, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0002000e, 0x00020002, 0x00020002, 0x00020002, 0x000e0002, 0x00000000,
    0x00000000, 0x00000000, 0x00020002, 0x00040004, 0x00080008, 0x00100010, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0008000c, 0x00080008, 0x00080008, 0x00080008, 0x000c0008, 0x00000000,
    0x00000000, 0x00000000, 0x00140008, 0x00000022, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x007f0000, 0x00000000,
    0x00000000, 0x00000000, 0x00080004, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00020003, 0x0022001e, 0x00220022, 0x001f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001c, 0x00020002, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00100018, 0x0011001e, 0x00110011, 0x003e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001c, 0x0002003e, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040018, 0x0004001e, 0x00040004, 0x001e0004, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0011003e, 0x00110011, 0x001e0011, 0x000e0010, 0x00000000,
    0x00000000, 0x00000000, 0x00020003, 0x0022001e, 0x00220022, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000008, 0x0008000e, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000010, 0x0010001e, 0x00100010, 0x00100010, 0x000e0010, 0x00000000,
    0x00000000, 0x00000000, 0x00020003, 0x000a0032, 0x000a0006, 0x00330012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0008000e, 0x00080008, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x002a0017, 0x002a002a, 0x007f002a, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0026001b, 0x00220022, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001c, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001f, 0x00220022, 0x001e0022, 0x00070002, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022007c, 0x00220022, 0x003c0022, 0x00700020, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x000c0036, 0x00040004, 0x001e0004, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001c, 0x0020001c, 0x001e0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040004, 0x0004001e, 0x00040004, 0x00180024, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00220033, 0x00220022, 0x006c0032, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00220077, 0x00140014, 0x00080008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00220077, 0x002a002a, 0x00140014, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00140036, 0x00080008, 0x00360014, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00220077, 0x00140014, 0x00080008, 0x000e0004, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0012003e, 0x00040008, 0x003e0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080010, 0x00080008, 0x00080004, 0x00080008, 0x00100008, 0x00000000,
    0x00000000, 0x00000000, 0x00080008, 0x00080008, 0x00080008, 0x00080008, 0x00080008, 0x00000000,
    0x00000000, 0x00000000, 0x00080004, 0x00080008, 0x00080010, 0x00080008, 0x00040008, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00260000, 0x00000019, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000008, 0x00080008, 0x00080008, 0x00080008, 0x00000000,
    0x00000000, 0x00000000, 0x00080000, 0x0022001c, 0x00220002, 0x0008001c, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012000c, 0x000f0002, 0x00020002, 0x001f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x002d0000, 0x00210012, 0x00120021, 0x0000002d, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00220077, 0x001c0014, 0x001c0008, 0x001c0008, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00080008, 0x00000008, 0x00080008, 0x00080008, 0x00000000, 0x00000000,
    0x00000000, 0x001c0000, 0x00040022, 0x0012000a, 0x00280024, 0x00200010, 0x001c0022, 0x00000000,
    0x00000000, 0x00000000, 0x00000014, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0042003c, 0x00850099, 0x00990085, 0x003c0042, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0011000e, 0x0011001e, 0x0000002e, 0x0000003f, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00240000, 0x00090012, 0x00240012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x0040007f, 0x00000040, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x0000003e, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0042003c, 0x00a5009d, 0x00a5009d, 0x003c0042, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0000001e, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012000c, 0x000c0012, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00080008, 0x0008003e, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x0010001c, 0x0004001c, 0x0000001c, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x0010001c, 0x0010001c, 0x0000001c, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040008, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00120012, 0x00120012, 0x002e0012, 0x00010002, 0x00000000,
    0x00000000, 0x00000000, 0x0015003e, 0x00150015, 0x00140016, 0x00140014, 0x00360014, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000004, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00180008, 0x000c0010, 0x00000000,
    0x00000000, 0x000c0008, 0x00080008, 0x0000001c, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0011000e, 0x00110011, 0x0000000e, 0x0000001f, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00090000, 0x00240012, 0x00090012, 0x00000000, 0x00000000,
    0x00000000, 0x00030002, 0x00220042, 0x00580017, 0x00720054, 0x00400041, 0x00000000, 0x00000000,
    0x00000000, 0x00030002, 0x00220042, 0x00780017, 0x00720044, 0x00700011, 0x00000000, 0x00000000,
    0x00000000, 0x00040007, 0x00240047, 0x00580017, 0x00720054, 0x00400041, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000008, 0x00080008, 0x00020004, 0x001c0022, 0x00000000,
    0x00000000, 0x00040002, 0x0008000c, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00080010, 0x0008000c, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00140008, 0x0008002e, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00190026, 0x0008000c, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000014, 0x0008000c, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x0022001c, 0x0008001c, 0x00140008, 0x003e0014, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x004c007e, 0x003a002a, 0x0009002e, 0x007b0049, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022003c, 0x00010001, 0x00010001, 0x001c0022, 0x00040008, 0x00000000,
    0x00000000, 0x00080004, 0x0022003f, 0x000e000a, 0x0002000a, 0x003f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00080010, 0x0022003f, 0x000e000a, 0x0022000a, 0x003f0022, 0x00000000, 0x00000000,
    0x00000000, 0x000a0004, 0x0022003f, 0x000e000a, 0x0002000a, 0x003f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000012, 0x0022003f, 0x000e000a, 0x0002000a, 0x003f0022, 0x00000000, 0x00000000,
    0x00000000, 0x00080004, 0x0008003e, 0x00080008, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00080010, 0x0008003e, 0x00080008, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00140008, 0x0008003e, 0x00080008, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000014, 0x0008003e, 0x00080008, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012000f, 0x00220022, 0x00220027, 0x000f0012, 0x00000000, 0x00000000,
    0x00000000, 0x001a002c, 0x00260073, 0x002a0026, 0x0032002a, 0x00270032, 0x00000000, 0x00000000,
    0x00000000, 0x00080004, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x00040008, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x0012000c, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x000d0016, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x00000012, 0x0012000c, 0x00210021, 0x00210021, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00220000, 0x00080014, 0x00220014, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0012002c, 0x00290029, 0x00250025, 0x000d0012, 0x00000000, 0x00000000,
    0x00000000, 0x00080004, 0x00220077, 0x00220022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00080010, 0x00220077, 0x00220022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00140008, 0x00220077, 0x00220022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000014, 0x00220077, 0x00220022, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00100000, 0x00000008, 0x00220077, 0x00080014, 0x00080008, 0x001c0008, 0x00000000, 0x00000000,
    0x00000000, 0x00020000, 0x001e0002, 0x00220022, 0x001e0022, 0x00020002, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0022001c, 0x00120022, 0x00220022, 0x0013002a, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040002, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040008, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00040000, 0x0000000a, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00160000, 0x0000000d, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x0000000a, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00040000, 0x0004000a, 0x0011000e, 0x0011001e, 0x002e0011, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00490036, 0x0009007e, 0x00360049, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001c, 0x00020002, 0x001c0022, 0x00040008, 0x00000000,
    0x00000000, 0x00000000, 0x00080004, 0x0022001c, 0x0002003e, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080010, 0x0022001c, 0x0002003e, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00000014, 0x0022001c, 0x0002003e, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000014, 0x0022001c, 0x0002003e, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00040000, 0x00000008, 0x0008000e, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00000004, 0x0008000e, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00000014, 0x0008000e, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000014, 0x0008000e, 0x00080008, 0x003e0008, 0x00000000, 0x00000000,
    0x00000000, 0x00140002, 0x0018000e, 0x00220024, 0x00220022, 0x000c0012, 0x00000000, 0x00000000,
    0x00000000, 0x002c0000, 0x0000001a, 0x0026001b, 0x00220022, 0x00770022, 0x00000000, 0x00000000,
    0x00000000, 0x00040000, 0x00000008, 0x0022001c, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00100000, 0x00000008, 0x0022001c, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00000014, 0x0022001c, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x002c0000, 0x0000001a, 0x0022001c, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000014, 0x0022001c, 0x00220022, 0x001c0022, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000008, 0x0000003e, 0x00000008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x0022001c, 0x002a0032, 0x001c0026, 0x00000000, 0x00000000,
    0x00000000, 0x00040000, 0x00000008, 0x00220033, 0x00220022, 0x006c0032, 0x00000000, 0x00000000,
    0x00000000, 0x00100000, 0x00000008, 0x00220033, 0x00220022, 0x006c0032, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00000014, 0x00220033, 0x00220022, 0x006c0032, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000014, 0x00220033, 0x00220022, 0x006c0032, 0x00000000, 0x00000000,
    0x00000000, 0x00080000, 0x00000004, 0x00220077, 0x00140014, 0x00080008, 0x000e0004, 0x00000000,
    0x00000000, 0x00020000, 0x001e0002, 0x00220022, 0x00220022, 0x001e0022, 0x00020002, 0x00000000,
    0x00000000, 0x00000000, 0x00000014, 0x00220077, 0x00140014, 0x00080008, 0x000e0004, 0x00000000,
    0x00000000, 0x00000000, 0x004a007c, 0x00390029, 0x00090029, 0x007c004a, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00490036, 0x00090079, 0x00360049, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080004, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00040008, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00080000, 0x00000004, 0x00000000,
    0x00000000, 0x00080000, 0x00100008, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00120009, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00090012, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00120000, 0x00000009, 0x00000000,
    0x00000000, 0x00120000, 0x00240012, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080008, 0x0008003e, 0x00080008, 0x00080008, 0x00080008, 0x00000000,
    0x00000000, 0x00000000, 0x00080008, 0x0008003e, 0x00080008, 0x003e0008, 0x00080008, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x001f000e, 0x001f001f, 0x0000000e, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x002a0000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00440038, 0x001f0002, 0x001f0002, 0x00380044, 0x00000000, 0x00000000,
    0x00000000, 0x001c0008, 0x002a0036, 0x0077006f, 0x0036003e, 0x0008001c, 0x00000000, 0x00000000,
];

/// Widths for proportional glyphs
pub const WIDTHS: [u8; 207] = [
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
8,
7,
7,
7,
7,
8,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
7,
8,
];

