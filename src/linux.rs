#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use std::fs::File;
use std::ffi::c_long;

use rustix::fs::fstatfs;

use crate::{FileSystemKind, FileSystemName};

const AAFS_MAGIC: c_long = 0x5a3c_69f0;
const ANON_INODE_FS_MAGIC: c_long = 0x0904_1934;
const BDEVFS_MAGIC: c_long = 0x6264_6576;
const BEFS_SUPER_MAGIC: c_long = 0x4246_5331;
const BFS_MAGIC: c_long = 0x1bad_face;
const BINFMTFS_MAGIC: c_long = 0x4249_4e4d;
const BTRFS_TEST_MAGIC: c_long = 0x7372_7279;
const CIFS_MAGIC_NUMBER: c_long = 0xff53_4d42;
const COH_SUPER_MAGIC: c_long = 0x012f_f7b7;
const CONFIGFS_MAGIC: c_long = 0x6265_6570;
const DEVFS_SUPER_MAGIC: c_long = 0x1373;
const EFIVARFS_MAGIC: c_long = 0xde5e_81e4;
const EXT_SUPER_MAGIC: c_long = 0x137d;
const EXT2_OLD_SUPER_MAGIC: c_long = 0xef51;
const FUSE_CTL_SUPER_MAGIC: c_long = 0x6573_5543;
const HFS_SUPER_MAGIC: c_long = 0x4244;
const JFS_SUPER_MAGIC: c_long = 0x3153_464a;
const MQUEUE_MAGIC: c_long = 0x1980_0202;
const MTD_INODE_FS_MAGIC: c_long = 0x1130_7854;
const NTFS_SB_MAGIC: c_long = 0x0534_6544e;
const PIPEFS_MAGIC: c_long = 0x5049_5045;
const PSTOREFS_MAGIC: c_long = 0x6165_676c;
const RAMFS_MAGIC: c_long = 0x8584_58f6;
const ROMFS_MAGIC: c_long = 0x7275;
const SMB2_MAGIC_NUMBER: c_long = 0xfe53_4d42;
const SOCKFS_MAGIC: c_long = 0x534f_434b;
const SQUASHFS_MAGIC: c_long = 0x7371_7368;
const SYSV2_SUPER_MAGIC: c_long = 0x012f_f7b6;
const SYSV4_SUPER_MAGIC: c_long = 0x012f_f7b5;
const UFS_MAGIC: c_long = 0x0001_1954;
const V9FS_MAGIC: c_long = 0x0102_1997;
const VXFS_SUPER_MAGIC: c_long = 0xa501_fcf5;
const XENIX_SUPER_MAGIC: c_long = 0x012f_f7b4;
const _XIAFS_SUPER_MAGIC: c_long = 0x012f_d16d;
const ZFS_SUPER_MAGIC: c_long = 0x2fc1_2fc1;

#[inline]
pub fn filesystem(file: &File) -> Result<FileSystemKind, std::io::Error> {
    let statfs = fstatfs(file)?;
    let f_type = statfs.f_type;

    let recognized = match f_type {
        AAFS_MAGIC => Some(FileSystemName::AppArmorFs),
        libc::ADFS_SUPER_MAGIC => None, // todo
        libc::AFFS_SUPER_MAGIC => None, // todo
        libc::AFS_SUPER_MAGIC => None, // todo
        ANON_INODE_FS_MAGIC => None, // todo
        libc::AUTOFS_SUPER_MAGIC => Some(FileSystemName::Autofs),
        BDEVFS_MAGIC => None, // todo
        BEFS_SUPER_MAGIC => None, // todo
        BFS_MAGIC => None, // todo
        BINFMTFS_MAGIC => Some(FileSystemName::Binfmtfs),
        libc::BPF_FS_MAGIC => None, // todo
        libc::BTRFS_SUPER_MAGIC => None, // todo
        BTRFS_TEST_MAGIC => None, // todo
        libc::CGROUP_SUPER_MAGIC => Some(FileSystemName::Cgroup),
        libc::CGROUP2_SUPER_MAGIC => Some(FileSystemName::Cgroup2),
        CIFS_MAGIC_NUMBER => None, // todo
        libc::CODA_SUPER_MAGIC => None, // todo
        COH_SUPER_MAGIC => None, // todo
        CONFIGFS_MAGIC => None, // todo
        libc::CRAMFS_MAGIC => None, // todo
        libc::DEBUGFS_MAGIC => None, // todo
        DEVFS_SUPER_MAGIC => Some(FileSystemName::Devfs),
        libc::DEVPTS_SUPER_MAGIC => Some(FileSystemName::Devpts),
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
        FUSE_CTL_SUPER_MAGIC => None, // todo
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
        libc::NSFS_MAGIC => Some(FileSystemName::Nsfs),
        NTFS_SB_MAGIC => None, // todo
        libc::OCFS2_SUPER_MAGIC => None, // todo
        libc::OPENPROM_SUPER_MAGIC => None, // todo
        libc::OVERLAYFS_SUPER_MAGIC => None, // todo
        PIPEFS_MAGIC => None, // todo
        libc::PROC_SUPER_MAGIC => Some(FileSystemName::Procfs),
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
        libc::TMPFS_MAGIC => Some(FileSystemName::Tmpfs),
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
