#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::ffi::CStr;
use std::fs::File;

use rustix::fs::fstatfs;

use crate::{FileSystemKind, FileSystemName};

#[inline]
pub fn filesystem(file: &File) -> Result<FileSystemKind, std::io::Error> {
    let statfs = fstatfs(file)?;

    // On success, `fstatfs` returns `0`
    // and sets `statfsbuf.f_fstypename` to a null-terminated C string.
    let f_fstypename = statfs.f_fstypename.map(|x| 0u8.wrapping_add_signed(x));
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
