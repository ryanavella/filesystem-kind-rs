#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::fs::File;
use std::ffi::c_long;

use rustix::fs::fstatfs;

use crate::{FileSystemKind, FileSystemName};

const ANON_INODE_FS_MAGIC: c_long = 0x9041934;
const BDEVFS_MAGIC: c_long = 0x62646576;
const BEFS_SUPER_MAGIC: c_long = 0x42465331;
const BFS_MAGIC: c_long = 0x1badface;
const BINFMTFS_MAGIC: c_long = 0x42494e4d;
const BTRFS_TEST_MAGIC: c_long = 0x73727279;
const CIFS_MAGIC_NUMBER: c_long = 0xff534d42;
const COH_SUPER_MAGIC: c_long = 0x12ff7b7;
const DEVFS_SUPER_MAGIC: c_long = 0x1373;
const EFIVARFS_MAGIC: c_long = 0xde5e81e4;
const EXT_SUPER_MAGIC: c_long = 0x137d;
const EXT2_OLD_SUPER_MAGIC: c_long = 0xef51;
const HFS_SUPER_MAGIC: c_long = 0x4244;
const JFS_SUPER_MAGIC: c_long = 0x3153464a;
const MQUEUE_MAGIC: c_long = 0x19800202;
const MTD_INODE_FS_MAGIC: c_long = 0x11307854;
const NTFS_SB_MAGIC: c_long = 0x5346544e;
const PIPEFS_MAGIC: c_long = 0x50495045;
const PSTOREFS_MAGIC: c_long = 0x6165676c;
const RAMFS_MAGIC: c_long = 0x858458f6;
const ROMFS_MAGIC: c_long = 0x7275;
const SMB2_MAGIC_NUMBER: c_long = 0xfe534d42;
const SOCKFS_MAGIC: c_long = 0x534f434b;
const SQUASHFS_MAGIC: c_long = 0x73717368;
const SYSV2_SUPER_MAGIC: c_long = 0x12ff7b6;
const SYSV4_SUPER_MAGIC: c_long = 0x12ff7b5;
const UFS_MAGIC: c_long = 0x11954;
const V9FS_MAGIC: c_long = 0x1021997;
const VXFS_SUPER_MAGIC: c_long = 0xa501fcf5;
const XENIX_SUPER_MAGIC: c_long = 0x12ff7b4;
const _XIAFS_SUPER_MAGIC: c_long = 0x12fd16d;
const ZFS_SUPER_MAGIC: c_long = 0x2fc12fc1;

#[inline]
pub fn filesystem(file: &File) -> Result<FileSystemKind, std::io::Error> {
    let statfs = fstatfs(file)?;
    let f_type = statfs.f_type;

    let recognized = match f_type {
        libc::ADFS_SUPER_MAGIC => None, // todo
        libc::AFFS_SUPER_MAGIC => None, // todo
        libc::AFS_SUPER_MAGIC => None, // todo
        ANON_INODE_FS_MAGIC => None, // todo
        libc::AUTOFS_SUPER_MAGIC => Some(FileSystemName::Autofs),
        BDEVFS_MAGIC => None, // todo
        BEFS_SUPER_MAGIC => None, // todo
        BFS_MAGIC => None, // todo
        BINFMTFS_MAGIC => None, // todo
        libc::BPF_FS_MAGIC => None, // todo
        libc::BTRFS_SUPER_MAGIC => None, // todo
        BTRFS_TEST_MAGIC => None, // todo
        libc::CGROUP_SUPER_MAGIC => None, // todo
        libc::CGROUP2_SUPER_MAGIC => None, // todo
        CIFS_MAGIC_NUMBER => None, // todo
        libc::CODA_SUPER_MAGIC => None, // todo
        COH_SUPER_MAGIC => None, // todo
        libc::CRAMFS_MAGIC => None, // todo
        libc::DEBUGFS_MAGIC => None, // todo
        DEVFS_SUPER_MAGIC => Some(FileSystemName::Devfs),
        libc::DEVPTS_SUPER_MAGIC => None, // todo
        libc::ECRYPTFS_SUPER_MAGIC => None, // todo
        EFIVARFS_MAGIC => None, // todo
        libc::EFS_SUPER_MAGIC => None, // todo
        EXT_SUPER_MAGIC => None, // todo
        EXT2_OLD_SUPER_MAGIC => None, // todo
        libc::EXT2_SUPER_MAGIC => {
            todo!("how to distinguish whether this is EXT2, EXT3, or EXT4?")
        }
        libc::F2FS_SUPER_MAGIC => None, // todo
        libc::FUSE_SUPER_MAGIC => Some(FileSystemName::Fusefs),
        libc::FUTEXFS_SUPER_MAGIC => None, // todo
        HFS_SUPER_MAGIC => None, // todo
        libc::HOSTFS_SUPER_MAGIC => None, // todo
        libc::HPFS_SUPER_MAGIC => None, // todo
        libc::HUGETLBFS_MAGIC => None, // todo
        libc::ISOFS_SUPER_MAGIC => None, // todo
        libc::JFFS2_SUPER_MAGIC => None, // todo
        JFS_SUPER_MAGIC => None, // todo
        libc::MINIX_SUPER_MAGIC => None, // todo
        libc::MINIX_SUPER_MAGIC2 => None, // todo
        libc::MINIX2_SUPER_MAGIC => None, // todo
        libc::MINIX2_SUPER_MAGIC2 => None, // todo
        libc::MINIX3_SUPER_MAGIC => None, // todo
        MQUEUE_MAGIC => Some(FileSystemName::Mqueuefs),
        libc::MSDOS_SUPER_MAGIC => Some(FileSystemName::Fat),
        MTD_INODE_FS_MAGIC => None, // todo
        libc::NCP_SUPER_MAGIC => None, // todo
        libc::NFS_SUPER_MAGIC => Some(FileSystemName::Nfs),
        libc::NILFS_SUPER_MAGIC => None, // todo
        libc::NSFS_MAGIC => None, // todo
        NTFS_SB_MAGIC => None, // todo
        libc::OCFS2_SUPER_MAGIC => None, // todo
        libc::OPENPROM_SUPER_MAGIC => None, // todo
        libc::OVERLAYFS_SUPER_MAGIC => None, // todo
        PIPEFS_MAGIC => None, // todo
        libc::PROC_SUPER_MAGIC => None, // todo
        PSTOREFS_MAGIC => None, // todo
        libc::QNX4_SUPER_MAGIC => None, // todo
        libc::QNX6_SUPER_MAGIC => None, // todo
        RAMFS_MAGIC => None, // todo
        libc::REISERFS_SUPER_MAGIC => None, // todo
        ROMFS_MAGIC => None, // todo
        libc::SECURITYFS_MAGIC => None, // todo
        libc::SELINUX_MAGIC => None, // todo
        libc::SMACK_MAGIC => None, // todo
        libc::SMB_SUPER_MAGIC => None, // todo
        SMB2_MAGIC_NUMBER => None, // todo
        SOCKFS_MAGIC => None, // todo
        SQUASHFS_MAGIC => None, // todo
        libc::SYSFS_MAGIC => None, // todo
        SYSV2_SUPER_MAGIC => None, // todo
        SYSV4_SUPER_MAGIC => None, // todo
        libc::TMPFS_MAGIC => None, // todo
        libc::TRACEFS_MAGIC => None, // todo
        libc::UDF_SUPER_MAGIC => None, // todo
        UFS_MAGIC => Some(FileSystemName::Ufs),
        libc::USBDEVICE_SUPER_MAGIC => None, // todo
        V9FS_MAGIC => None, // todo
        VXFS_SUPER_MAGIC => None, // todo
        libc::XENFS_SUPER_MAGIC => None, // todo
        XENIX_SUPER_MAGIC => None, // todo
        libc::XFS_SUPER_MAGIC => None, // todo
        _XIAFS_SUPER_MAGIC => None, // todo
        ZFS_SUPER_MAGIC => Some(FileSystemName::Zfs),
        _ => None,
    };
    Ok(recognized.map_or_else(
        || FileSystemKind::Unrecognized(Box::new(format!("{f_type:#X}"))),
        FileSystemKind::Recognized,
    ))
}
