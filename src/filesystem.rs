use std::io;
use std::path::Path;

#[cfg(target_os = "linux")]
pub fn get_disk_usage<P: AsRef<Path>>(path: P) -> io::Result<(u64, u64)> {
    use nix::sys::statvfs::statvfs;

    let stats = statvfs(path.as_ref())?;
    let free = stats.blocks_free() * stats.block_size();
    let total = stats.blocks() * stats.block_size();

    Ok((free as u64, total as u64))
}

#[cfg(target_os = "windows")]
pub fn get_disk_usage<P: AsRef<Path>>(path: P) -> io::Result<(u64, u64)> {
    use std::mem;
    use std::ptr;
    use winapi::um::fileapi::GetDiskFreeSpaceExW;
    use winapi::um::winnt::ULARGE_INTEGER;

    unsafe {
        let mut free_bytes_available: ULARGE_INTEGER = mem::zeroed();
        let mut total_number_of_bytes: ULARGE_INTEGER = mem::zeroed();
        let mut total_number_of_free_bytes: ULARGE_INTEGER = mem::zeroed();

        let path_wide: Vec<u16> = path
            .as_ref()
            .to_str()
            .unwrap()
            .encode_utf16()
            .chain(Some(0))
            .collect();

        let success = GetDiskFreeSpaceExW(
            path_wide.as_ptr(),
            &mut free_bytes_available,
            &mut total_number_of_bytes,
            &mut total_number_of_free_bytes,
        );

        if success != 0 {
            Ok((
                total_number_of_free_bytes.QuadPart(),
                total_number_of_bytes.QuadPart(),
            ))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_disk_usage<P: AsRef<Path>>(path: P) -> io::Result<(u64, u64)> {
    use nix::sys::statvfs::statvfs;
    use std::fs;

    let stats = statvfs(path.as_ref())?;
    let free = stats.blocks_free() * stats.block_size();
    let total = stats.blocks() * stats.block_size();

    Ok((free as u64, total as u64))
}

#[cfg(target_os = "freebsd")]
pub fn get_disk_usage<P: AsRef<Path>>(path: P) -> io::Result<(u64, u64)> {
    use nix::sys::statvfs::statvfs;
    use std::fs;

    let stats = statvfs(path.as_ref())?;
    let free = stats.blocks_free() * stats.block_size();
    let total = stats.blocks() * stats.block_size();

    Ok((free as u64, total as u64))
}

pub fn is_special_filesystem(path: &str) -> bool {
    path.starts_with("/proc") || path.starts_with("/sys")
}
