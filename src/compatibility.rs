#[cfg(target_os = "linux")]
pub fn is_supported_platform() -> bool {
    true // Linux est pris en charge
}

#[cfg(target_os = "windows")]
pub fn is_supported_platform() -> bool {
    true // Windows est pris en charge
}

#[cfg(target_os = "macos")]
pub fn is_supported_platform() -> bool {
    true // macOS est pris en charge
}

#[cfg(target_os = "freebsd")]
pub fn is_supported_platform() -> bool {
    true // FreeBSD est pris en charge
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "windows",
    target_os = "macos",
    target_os = "freebsd"
)))]
pub fn is_supported_platform() -> bool {
    false // Plateforme non prise en charge
}
