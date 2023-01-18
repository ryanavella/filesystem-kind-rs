#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::ffi::CStr;
use std::fs::File;
use std::os::fd::AsRawFd;

use crate::{FileSystemKind, FileSystemName};

#[inline]
pub fn filesystem(file: &File) -> Result<FileSystemKind, std::io::Error> {
    // SAFETY:
    //   `struct statfs` can be safely zero-initialized.
    //   We could even leave it uninitialized,
    //   but this reduces the scope of undefined behavior
    //   in the unlikely scenario where `fstatfs` had a logic bug
    let mut statfsbuf: libc::statfs = unsafe { std::mem::zeroed() };

    let fd = file.as_raw_fd();

    // SAFETY:
    //   `fd` is a raw file descriptor corresponding to `file`.
    //   The caller passes `file` by shared reference (`&'_ File`),
    //   so it is guaranteed to be alive and valid for the duration of the callee.
    //   We only inspect the contents of `statfsbuf` in the case that `fstatfs` returns `0`,
    //   upon which `fstatfs` guarantees that `statfsbuf` is initialized to meaningful values.
    //   Even if FreeBSD's libc was buggy and `fstatfs` had a logic bug,
    //   we don't blindly trust that `statfsbuf.f_fstypename` contains a null byte.
    let ret = unsafe { libc::fstatfs(fd, &mut statfsbuf) };
    match ret {
        0 => {
            // On success, `fstatfs` returns `0`
            // and sets `statfsbuf.f_fstypename` to a null-terminated C string.
            let f_fstypename = statfsbuf.f_fstypename.map(|x| 0u8.wrapping_add_signed(x));
            let fstypename = CStr::from_bytes_with_nul(
                f_fstypename
                    .split_inclusive(|x| *x == 0)
                    .next()
                    .expect("fstatfs returned an empty string for f_fstypename"),
            )
            .expect("fstatfs returned a non-null-terminated string for f_fstypename")
            .to_str()
            .expect("fstatfs returned a non-UTF-8 string for f_fstypename");
            let recognized = match fstypename {
                "autofs" => Some(FileSystemName::Autofs),
                "devfs" => Some(FileSystemName::Devfs),
                "ext2fs" => Some(FileSystemName::Ext2),
                "ext3fs" => Some(FileSystemName::Ext3),
                "ext4fs" => Some(FileSystemName::Ext4),
                "fdescfs" => Some(FileSystemName::Fdescfs),
                "fusefs" => Some(FileSystemName::Fusefs),
                "linprocfs" => Some(FileSystemName::Linprocfs),
                "linsysfs" => Some(FileSystemName::Linsysfs),
                "mqueuefs" => Some(FileSystemName::Mqueuefs),
                "msdosfs" => Some(FileSystemName::Fat),
                "nfs" => Some(FileSystemName::Nfs),
                "nullfs" => Some(FileSystemName::Nullfs),
                "procfs" => Some(FileSystemName::Procfs),
                "tmpfs" => Some(FileSystemName::Tmpfs),
                "ufs" => Some(FileSystemName::Ufs),
                "zfs" => Some(FileSystemName::Zfs),
                _ => None,
            };
            Ok(recognized.map_or_else(
                || FileSystemKind::Unrecognized(Box::new(fstypename.into())),
                FileSystemKind::Recognized,
            ))
        }
        -1 => {
            // On failure, `fstatfs` returns `-1` and sets `errno` accordingly.
            Err(std::io::Error::last_os_error())
        }
        code => {
            // This branch is only possible if the implementation of `fstatfs` is buggy,
            // i.e. if it is not POSIX-compliant.
            panic!("fstatfs failed with unexpected return value {code}")
        }
    }
}
