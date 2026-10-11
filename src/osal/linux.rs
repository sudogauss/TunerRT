use std::path::{Path};
use std::fs::{self};

fn read_system_file(p: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(p).ok().map(|s| s.trim().to_string())
}

pub(super) fn read_raw_cpuinfo() -> Result<String, &'static str> {
    match read_system_file("/proc/cpuinfo") {
        Some(s) => Ok(s),
        None => Err("Cannot state /proc/cpuinfo")
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    
    #[test]
    fn read_cpuinfo_correct() { assert!(read_system_file("/proc/cpuinfo").is_some()); }

    #[test]
    fn read_cpuinfo_cincorrect() { assert!(read_system_file("/proc/cpux").is_none()); }
}