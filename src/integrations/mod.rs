#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;
#[cfg(all(not(target_os = "linux"), not(target_os = "android"), not(target_os = "windows")))]
mod osx;
#[cfg(target_os = "windows")]
mod windows;

mod shared_api;
pub use self::shared_api::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;
#[cfg(all(not(target_os = "linux"), not(target_os = "android"), not(target_os = "windows")))]
pub use self::osx::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;
