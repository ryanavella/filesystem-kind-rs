#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::ffi::OsString;
use std::fs::File;
use std::os::windows::ffi::OsStringExt;
use std::ptr;

use std::os::windows::io::AsRawHandle;
use winapi::shared::minwindef::MAX_PATH;

use crate::{FileSystemKind, FileSystemName};

#[inline]
pub fn filesystem(file: &File) -> Result<FileSystemKind, std::io::Error> {
    // todo: This is a fraction of a kB, should this be stack allocated instead?
    let mut name_buf = vec![0; MAX_PATH + 1];

    let h_file = file.as_raw_handle();
    let buf_ptr = name_buf.as_mut_ptr();
    let buf_len = name_buf.len().try_into().unwrap();

    // SAFETY:
    //   `h_file` is a raw handle corresponding to `file`.
    //   The caller passes `file` by shared reference (`&'_ File`),
    //   so it is guaranteed to be alive and valid for the duration of the callee.
    //   Per the documentation of `GetVolumeInformationByHandleW`,
    //   we pass `0` and `NULL` for the information we are uninterested in,
    //   and we pass a buffer with at least `MAX_PATH + 1` wide characters for
    //   the returned file system name.
    let ret = unsafe {
        winapi::um::fileapi::GetVolumeInformationByHandleW(
            h_file,          // hFile: HANDLE
            ptr::null_mut(), // lpVolumeNameBuffer: LPWSTR
            0,               // nVolumeNameSize: DWORD
            ptr::null_mut(), // lpVolumeSerialNumber: LPDWORD
            ptr::null_mut(), // lpMaximumComponentLength: LPDWORD
            ptr::null_mut(), // lpFileSystemFlags: LPDWORD
            buf_ptr,         // lpFileSystemNameBuffer: LPWSTR
            buf_len,         // nFileSystemNameSize: DWORD
        )
    };
    if ret == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        // If `GetVolumeInformationByHandleW` is successful,
        // it returns a non-zero value.
        let fs_name = OsString::from_wide(
            name_buf
            .split(|x| *x == 0)
            .next()
            .expect("GetVolumeInformationByHandleW returned an empty string for lpFileSystemNameBuffer")
        );
        let fs_name = fs_name
            .as_os_str()
            .to_str()
            .expect("GetVolumeInformationByHandleW returned a non-UTF-8 string for lpFileSystemNameBuffer");
        // todo: can `GetVolumeInformationByHandleW` return filesystems other than these?
        let recognized = match fs_name {
            "BTRFS" => Some(FileSystemName::Btrfs),
            "CDFS" => Some(FileSystemName::Cdfs),
            "EXT2" => Some(FileSystemName::Ext2),
            "EXT3" => Some(FileSystemName::Ext3),
            "EXT4" => Some(FileSystemName::Ext4),
            "FAT" => Some(FileSystemName::Fat),
            "FAT32" => Some(FileSystemName::Fat32),
            "FATX" => Some(FileSystemName::Fatx),
            "NTFS" => Some(FileSystemName::Ntfs),
            _ => None,
        };
        Ok(recognized.map_or_else(
            || FileSystemKind::Unrecognized(Box::new(fs_name.into())),
            FileSystemKind::Recognized,
        ))
    }
}
