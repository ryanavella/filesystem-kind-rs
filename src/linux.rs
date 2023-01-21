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

    match statfs.f_type {
        libc::ADFS_SUPER_MAGIC => todo!(),
        libc::AFFS_SUPER_MAGIC => todo!(),
        libc::AFS_SUPER_MAGIC => todo!(),
        ANON_INODE_FS_MAGIC => todo!(),
        libc::AUTOFS_SUPER_MAGIC => todo!(),
        BDEVFS_MAGIC => todo!(),
        BEFS_SUPER_MAGIC => todo!(),
        BFS_MAGIC => todo!(),
        BINFMTFS_MAGIC => todo!(),
        libc::BPF_FS_MAGIC => todo!(),
        libc::BTRFS_SUPER_MAGIC => todo!(),
        BTRFS_TEST_MAGIC => todo!(),
        libc::CGROUP_SUPER_MAGIC => todo!(),
        libc::CGROUP2_SUPER_MAGIC => todo!(),
        CIFS_MAGIC_NUMBER => todo!(),
        libc::CODA_SUPER_MAGIC => todo!(),
        COH_SUPER_MAGIC => todo!(),
        libc::CRAMFS_MAGIC => todo!(),
        libc::DEBUGFS_MAGIC => todo!(),
        DEVFS_SUPER_MAGIC => todo!(),
        libc::DEVPTS_SUPER_MAGIC => todo!(),
        libc::ECRYPTFS_SUPER_MAGIC => todo!(),
        EFIVARFS_MAGIC => todo!(),
        libc::EFS_SUPER_MAGIC => todo!(),
        EXT_SUPER_MAGIC => todo!(),
        EXT2_OLD_SUPER_MAGIC => todo!(),
        libc::EXT2_SUPER_MAGIC => todo!(),
        libc::EXT3_SUPER_MAGIC => todo!(),
        libc::EXT4_SUPER_MAGIC => todo!(),
        libc::F2FS_SUPER_MAGIC => todo!(),
        libc::FUSE_SUPER_MAGIC => todo!(),
        libc::FUTEXFS_SUPER_MAGIC => todo!(),
        HFS_SUPER_MAGIC => todo!(),
        libc::HOSTFS_SUPER_MAGIC => todo!(),
        libc::HPFS_SUPER_MAGIC => todo!(),
        libc::HUGETLBFS_MAGIC => todo!(),
        libc::ISOFS_SUPER_MAGIC => todo!(),
        libc::JFFS2_SUPER_MAGIC => todo!(),
        JFS_SUPER_MAGIC => todo!(),
        libc::MINIX_SUPER_MAGIC => todo!(),
        libc::MINIX_SUPER_MAGIC2 => todo!(),
        libc::MINIX2_SUPER_MAGIC => todo!(),
        libc::MINIX2_SUPER_MAGIC2 => todo!(),
        libc::MINIX3_SUPER_MAGIC => todo!(),
        MQUEUE_MAGIC => todo!(),
        libc::MSDOS_SUPER_MAGIC => todo!(),
        MTD_INODE_FS_MAGIC => todo!(),
        libc::NCP_SUPER_MAGIC => todo!(),
        libc::NFS_SUPER_MAGIC => todo!(),
        libc::NILFS_SUPER_MAGIC => todo!(),
        libc::NSFS_MAGIC => todo!(),
        NTFS_SB_MAGIC => todo!(),
        libc::OCFS2_SUPER_MAGIC => todo!(),
        libc::OPENPROM_SUPER_MAGIC => todo!(),
        libc::OVERLAYFS_SUPER_MAGIC => todo!(),
        PIPEFS_MAGIC => todo!(),
        libc::PROC_SUPER_MAGIC => todo!(),
        PSTOREFS_MAGIC => todo!(),
        libc::QNX4_SUPER_MAGIC => todo!(),
        libc::QNX6_SUPER_MAGIC => todo!(),
        RAMFS_MAGIC => todo!(),
        libc::REISERFS_SUPER_MAGIC => todo!(),
        ROMFS_MAGIC => todo!(),
        libc::SECURITYFS_MAGIC => todo!(),
        libc::SELINUX_MAGIC => todo!(),
        libc::SMACK_MAGIC => todo!(),
        libc::SMB_SUPER_MAGIC => todo!(),
        SMB2_MAGIC_NUMBER => todo!(),
        SOCKFS_MAGIC => todo!(),
        SQUASHFS_MAGIC => todo!(),
        libc::SYSFS_MAGIC => todo!(),
        SYSV2_SUPER_MAGIC => todo!(),
        SYSV4_SUPER_MAGIC => todo!(),
        libc::TMPFS_MAGIC => todo!(),
        libc::TRACEFS_MAGIC => todo!(),
        libc::UDF_SUPER_MAGIC => todo!(),
        UFS_MAGIC => todo!(),
        libc::USBDEVICE_SUPER_MAGIC => todo!(),
        V9FS_MAGIC => todo!(),
        VXFS_SUPER_MAGIC => todo!(),
        libc::XENFS_SUPER_MAGIC => todo!(),
        XENIX_SUPER_MAGIC => todo!(),
        libc::XFS_SUPER_MAGIC => todo!(),
        _XIAFS_SUPER_MAGIC => todo!(),
        ZFS_SUPER_MAGIC => todo!(),
        _ => todo!(),
    }
}
