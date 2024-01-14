// Versioning infrastructure
// This generates the SEMVER data that is displayed when quering `xous ver`
// It also generates timestamps, if demanded.

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::Command;

use chrono::Local;

pub(crate) fn generate_version(add_timestamp: bool) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "git describe --tags --long"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git describe --tags --long")
            .output()
            .expect("failed to execute process")
    };
    let gitver = output.stdout;
    let semver = String::from_utf8_lossy(&gitver);

    let version_file = "services/xous-ticktimer/src/version.rs";

    // Read the existing file to see if it needs to be updated.
    let mut existing_data = Vec::new();
    if let Ok(mut f) = std::fs::File::open(version_file) {
        f.read_to_end(&mut existing_data).ok();
    }

    let mut new_data = Vec::new();
    print_header(&mut new_data);
    if add_timestamp {
        let now = Local::now();
        write!(
            new_data,
            "#[allow(dead_code)]\npub const TIMESTAMP: &'static str = \"{}\";\n",
            now.to_rfc2822()
        )
        .expect("couldn't add our timestamp");
    } else {
        write!(new_data, "#[allow(dead_code)]\npub const TIMESTAMP: &'static str = \"unavailable\";\n")
            .expect("couldn't add our timestamp");
    }
    write!(
        new_data,
        "pub const SEMVER: &'static str = \"{}\";\n",
        semver.strip_suffix("\r\n").or(semver.strip_suffix("\n")).unwrap_or(&semver)
    )
    .expect("couldn't add our semver");

    if existing_data != new_data {
        let mut vfile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(version_file)
            .expect("Can't open our version file for writing");
        vfile.write_all(&mut new_data).expect("couldn't write new timestamp to version.rs");
    }
}

fn print_header<U: Write>(out: &mut U) {
    let s = r####"#![cfg_attr(rustfmt, rustfmt_skip)]
// Versioning information is kept in a separate file, attached to a small, well-known server in the Xous System
// This is a trade-off between rebuild times and flexibility.
// This was autogenerated by xtask/src/main.rs:print_header(). Do not edit manually.

pub(crate) fn get_version() -> crate::api::VersionString {
    let mut v = crate::api::VersionString {
        version: xous_ipc::String::new()
    };
    v.version.append(SEMVER).ok();
    v.version.append("\n").ok();
    v.version.append(TIMESTAMP).ok();
    v
}
"####;
    out.write_all(s.as_bytes()).expect("couldn't write our version template header");
}
