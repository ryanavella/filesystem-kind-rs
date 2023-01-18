#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::fs::File;

/// The kind of filesystem that a file resides on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileSystemKind {
    /// A recognized filesystem.
    Recognized(FileSystemName),
    /// An unrecognized filesystem.
    Unrecognized(Box<String>),
    // todo: is there a better way to keep this pointer-sized? `Box<str>` is 2 pointer sizes :(
    // todo: should this be `Option<Box<_>>`?
    // todo: what order should enum variants be in?
    // todo: add `Unsupported` variant for OS's where
    //   this information can't be easily queried?
}

/// An enum representing all recognized filesystem names.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
// todo: come up with naming conventions,
//   i.e. should they all end in "fs"?
//   Or should we stick to the common renderings of each fs?
pub enum FileSystemName {
    Autofs,
    Btrfs,
    Cdfs,
    Devfs,
    Ext2,
    Ext3,
    Ext4,
    Fat,
    Fat32,
    Fatx,
    Fdescfs,
    Fusefs,
    Linprocfs, // todo: should this just be procfs?
    Linsysfs, // todo: should this just be sysfs?
    Mqueuefs,
    Nfs,
    Ntfs,
    Nullfs,
    Procfs,
    Tmpfs,
    Ufs,
    Zfs,
}

/// An extension trait for `File` that allows querying the kind of filesystem it resides on.
// todo: is there a possible error condition that isn't covered by `std::io::Error`?
pub trait FileExt {
    /// Query the filesystem that this file resides on.
    /// 
    /// # Errors
    /// 
    /// Note that a variant of `std::io::Error` may be returned,
    /// which can indicate a number of error conditions
    /// such as corrupted data, insufficient permissions, etc.
    fn filesystem(&self) -> Result<FileSystemKind, std::io::Error>;
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "freebsd")] {
        mod freebsd;
        use freebsd::filesystem;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        use windows::filesystem;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        use macos::filesystem;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        use linux::filesystem;
    } else {
        // todo: ios, android, and other bsd's
        // todo: unix-y/posix-y fallback?
    }
}

impl FileExt for File {
    fn filesystem(&self) -> Result<FileSystemKind, std::io::Error> {
        filesystem(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_for_unrecognized_filesystems() {
        use std::collections::HashSet;
        use walkdir::WalkDir;

        let mut set = HashSet::new();

        for entry in WalkDir::new("/").into_iter().filter_map(Result::ok) {
            if let Ok(file) = File::open(entry.path()) {
                if let Ok(FileSystemKind::Unrecognized(s)) = file.filesystem() {
                    // panic!("unknown filesystem: {:?}", s);
                    set.insert(s);
                }
            }
        }

        if !set.is_empty() {
            panic!("{:?}", set);
        }
    }
}
