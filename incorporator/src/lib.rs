#![cfg(all(
    any(feature = "host", feature = "diff", feature = "explicit"),
    any(
        not(feature = "diff"),
        all(
            target_arch = "x86_64",
            any(target_os = "windows", target_os = "linux")
        )
    )
))]

use std::{io::Write, process::Command};

use tempfile::NamedTempFile;

#[cfg(feature = "host")]
pub static BIN: &[u8] = include_bytes!(env!("CARGO_BIN_FILE_BINDEP_HOST_bindep"));
#[cfg(feature = "diff")]
pub static BIN: &[u8] = include_bytes!(env!("CARGO_BIN_FILE_BINDEP_DIFF_bindep"));
#[cfg(feature = "explicit")]
pub static BIN: &[u8] = include_bytes!(env!("CARGO_BIN_FILE_BINDEP_EXPLICIT_bindep"));

pub fn enabled_in_lib() -> bool {
    featuredep::feature_enabled()
}

pub fn enabled_in_bin() -> bool {
    let mut bin = NamedTempFile::new().unwrap();
    bin.write_all(BIN).unwrap();
    #[cfg(unix)]
    {
        use std::{fs::Permissions, os::unix::fs::PermissionsExt};
        let mut permissions = bin.as_file().metadata().unwrap().permissions();
        permissions.set_mode(0o755);
        bin.as_file().set_permissions(permissions).unwrap();
    }
    let path = bin.into_temp_path();
    let was_enabled = Command::new(&path).output().unwrap().stdout;
    let was_enabled = std::str::from_utf8(&was_enabled).unwrap();
    was_enabled.trim().parse().expect(was_enabled)
}
