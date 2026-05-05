//! RFS — Raptor File System kernel driver.
//!
//! Read-only access to RFS_V1 disk images produced by `mkrfs`.
//! Block I/O is performed through the VirtIO block device (8 sectors = 1 block).
//!
//! ## Public API
//!
//! - [`init`]          — mount; call once after virtio-blk init. Returns `true` on success.
//! - [`open`]          — path → fd (≥ 0) or negative error code.
//! - [`read`]          — fd, buf → bytes read or negative error.
//! - [`close`]         — fd → 0 or negative error.
//! - [`stat_path`]     — path, stat_out → `true` on success.
//! - [`readdir_path`]  — path → `Option<Vec<DirEntry>>`.
//!
//! ## Error codes (returned as negative i64)
//!
//! | Value | Meaning                        |
//! |-------|--------------------------------|
//! |   -1  | No VirtIO block device         |
//! |   -4  | Invalid argument               |
//! |   -5  | No such file or directory      |
//! |   -6  | Bad file descriptor            |
//! |   -7  | Is a directory                 |
//! |   -8  | Not a directory                |
//! |   -9  | Filesystem not mounted         |
//! |  -10  | Too many open files            |

#![allow(dead_code)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::serial::SpinLock;
use crate::virtio_blk;

// ── Error codes ───────────────────────────────────────────────────────────────

pub const ENODEV:  i64 = -1;
pub const EINVAL:  i64 = -4;
pub const ENOENT:  i64 = -5;
pub const EBADF:   i64 = -6;
pub const EISDIR:  i64 = -7;
pub const ENOTDIR: i64 = -8;
pub const ENOMNT:  i64 = -9;
pub const EMFILE:  i64 = -10;

// ── On-disk constants ─────────────────────────────────────────────────────────

const BLOCK_SIZE:       usize = 4096;
const SECTORS_PER_BLK: u64   = (BLOCK_SIZE / virtio_blk::SECTOR_SIZE) as u64; // 8

const MAGIC: &[u8; 8] = b"RFS_V1\0\0";

const INODE_START:      u32   = 2;
const INODE_COUNT:      u32   = 1024;
const INODES_PER_BLOCK: u32   = (BLOCK_SIZE / INODE_SIZE) as u32; // 32
const INODE_SIZE:       usize = 128;

const INLINE_EXTENTS: usize = 4;
const OVFL_EXTENTS:   usize = 255;
const EXTENT_SIZE:    usize = 16;
const OVFL_HDR:       usize = 16; // next(4) + used(4) + _pad(8)

pub const INODE_USED:     u32 = 1 << 0;
pub const INODE_DIR:      u32 = 1 << 1;
pub const INODE_SYMLINK:  u32 = 1 << 2;
pub const INODE_FAST_SYM: u32 = 1 << 3;

pub const FT_REG:     u8 = 1;
pub const FT_DIR:     u8 = 2;
pub const FT_SYMLINK: u8 = 3;

const MAX_FDS:          usize = 64;
const MAX_SYMLINK_HOPS: usize = 8;

// ── Little-endian helpers ─────────────────────────────────────────────────────

#[inline]
fn get_u16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes(b[o..o+2].try_into().unwrap())
}
#[inline]
fn get_u32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o+4].try_into().unwrap())
}
#[inline]
fn get_u64(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o+8].try_into().unwrap())
}

// ── Block I/O ─────────────────────────────────────────────────────────────────

fn read_block(blk: u32) -> Option<[u8; BLOCK_SIZE]> {
    let base = blk as u64 * SECTORS_PER_BLK;
    let mut out = [0u8; BLOCK_SIZE];
    for i in 0..SECTORS_PER_BLK {
        let mut sector = [0u8; virtio_blk::SECTOR_SIZE];
        if !virtio_blk::read_sector(base + i, &mut sector) {
            return None;
        }
        let off = (i as usize) * virtio_blk::SECTOR_SIZE;
        out[off..off + virtio_blk::SECTOR_SIZE].copy_from_slice(&sector);
    }
    Some(out)
}

// ── Inode ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Inode {
    pub flags:        u32,
    pub mode:         u16,
    pub uid:          u32,
    pub gid:          u32,
    pub nlink:        u32,
    pub size:         u64,
    pub blocks:       u64,
    pub mtime:        u64,
    pub ctime:        u64,
    pub ovfl_block:   u32,
    pub extent_count: u16,
    // Raw packed extent bytes: [logical(4) physical(4) count(4) flags(4)] × 4
    extents: [[u8; EXTENT_SIZE]; INLINE_EXTENTS],
}

fn parse_inode(b: &[u8]) -> Inode {
    let mut extents = [[0u8; EXTENT_SIZE]; INLINE_EXTENTS];
    for (i, slot) in extents.iter_mut().enumerate() {
        let off = 60 + i * EXTENT_SIZE;
        slot.copy_from_slice(&b[off..off + EXTENT_SIZE]);
    }
    Inode {
        flags:        get_u32(b,  0),
        mode:         get_u16(b,  4),
        uid:          get_u32(b,  8),
        gid:          get_u32(b, 12),
        nlink:        get_u32(b, 16),
        size:         get_u64(b, 20),
        blocks:       get_u64(b, 28),
        mtime:        get_u64(b, 36),
        ctime:        get_u64(b, 44),
        ovfl_block:   get_u32(b, 52),
        extent_count: get_u16(b, 56),
        extents,
    }
}

fn read_inode(ino: u32) -> Option<Inode> {
    if ino >= INODE_COUNT { return None; }
    let blk = INODE_START + ino / INODES_PER_BLOCK;
    let buf = read_block(blk)?;
    let off = ((ino % INODES_PER_BLOCK) as usize) * INODE_SIZE;
    let inode = parse_inode(&buf[off..off + INODE_SIZE]);
    if inode.flags & INODE_USED == 0 { return None; }
    Some(inode)
}

// ── Extent traversal ──────────────────────────────────────────────────────────

/// Map logical block index → physical block number.
/// `None` = sparse hole; caller should zero-fill.
fn resolve_block(inode: &Inode, logical: u32) -> Option<u32> {
    let inline_count = (inode.extent_count as usize).min(INLINE_EXTENTS);

    for i in 0..inline_count {
        let e          = &inode.extents[i];
        let e_logical  = get_u32(e, 0);
        let e_physical = get_u32(e, 4);
        let e_count    = get_u32(e, 8);
        if logical >= e_logical && logical < e_logical + e_count {
            return Some(e_physical + (logical - e_logical));
        }
    }

    if (inode.extent_count as usize) <= INLINE_EXTENTS || inode.ovfl_block == 0 {
        return None;
    }

    let mut ovfl         = inode.ovfl_block;
    let mut seen_extents = inline_count;

    loop {
        let buf  = read_block(ovfl)?;
        let used = get_u32(&buf, 4) as usize;

        for i in 0..used.min(OVFL_EXTENTS) {
            let off        = OVFL_HDR + i * EXTENT_SIZE;
            let e_logical  = get_u32(&buf, off);
            let e_physical = get_u32(&buf, off + 4);
            let e_count    = get_u32(&buf, off + 8);
            if logical >= e_logical && logical < e_logical + e_count {
                return Some(e_physical + (logical - e_logical));
            }
        }

        seen_extents += used;
        if seen_extents >= inode.extent_count as usize { break; }

        let next = get_u32(&buf, 0);
        if next == 0 { break; }
        ovfl = next;
    }
    None
}

// ── File data read ────────────────────────────────────────────────────────────

/// Read up to `buf.len()` bytes from `inode` starting at byte `offset`.
/// Sparse holes are zero-filled. Returns bytes actually read.
pub fn read_file_data(inode: &Inode, offset: u64, buf: &mut [u8]) -> usize {
    if offset >= inode.size { return 0; }
    let avail    = (inode.size - offset) as usize;
    let to_read  = buf.len().min(avail);
    let mut done = 0usize;

    while done < to_read {
        let file_off = offset + done as u64;
        let logical  = (file_off / BLOCK_SIZE as u64) as u32;
        let blk_off  = (file_off % BLOCK_SIZE as u64) as usize;
        let chunk    = (BLOCK_SIZE - blk_off).min(to_read - done);

        if let Some(phys) = resolve_block(inode, logical) {
            match read_block(phys) {
                Some(b) => buf[done..done + chunk].copy_from_slice(&b[blk_off..blk_off + chunk]),
                None    => break,
            }
        } else {
            buf[done..done + chunk].fill(0); // sparse hole
        }
        done += chunk;
    }
    done
}

// ── Symlink target ────────────────────────────────────────────────────────────

fn read_symlink_target(inode: &Inode) -> Option<Vec<u8>> {
    let len = inode.size as usize;
    if inode.flags & INODE_FAST_SYM != 0 {
        // Target stored inline in the extents[] field (max 64 bytes).
        let capped = len.min(64);
        let mut out = Vec::with_capacity(capped);
        'done: for slot in &inode.extents {
            for &byte in slot {
                if out.len() >= capped { break 'done; }
                out.push(byte);
            }
        }
        Some(out)
    } else {
        let mut buf = alloc::vec![0u8; len];
        let n = read_file_data(inode, 0, &mut buf);
        if n < len { None } else { Some(buf) }
    }
}

// ── Directory scanning ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DirEntry {
    pub ino:       u32,
    pub file_type: u8,
    pub name:      String,
}

/// Return all live entries in directory `inode` (including `.` and `..`).
pub fn scan_dir(inode: &Inode) -> Vec<DirEntry> {
    let mut entries = Vec::new();
    let size        = inode.size as usize;
    if size == 0 { return entries; }
    let n_blocks = (size + BLOCK_SIZE - 1) / BLOCK_SIZE;

    for logical in 0..n_blocks as u32 {
        let phys = match resolve_block(inode, logical) {
            Some(p) => p,
            None    => continue,
        };
        let buf = match read_block(phys) {
            Some(b) => b,
            None    => continue,
        };

        // The last block may be partial; all others are full.
        let is_last    = (logical as usize + 1) * BLOCK_SIZE > size;
        let block_used = if is_last && size % BLOCK_SIZE != 0 { size % BLOCK_SIZE } else { BLOCK_SIZE };

        let mut pos = 0usize;
        while pos + 8 <= block_used {
            let ino       = get_u32(&buf, pos);
            let rec_len   = get_u16(&buf, pos + 4) as usize;
            let name_len  = buf[pos + 6] as usize;
            let file_type = buf[pos + 7];

            if rec_len == 0 { break; } // corrupted — stop

            if ino != 0 && name_len > 0 && pos + 8 + name_len <= BLOCK_SIZE {
                let raw = &buf[pos + 8..pos + 8 + name_len];
                if let Ok(s) = core::str::from_utf8(raw) {
                    entries.push(DirEntry { ino, file_type, name: String::from(s) });
                }
            }
            pos += rec_len;
        }
    }
    entries
}

fn lookup_in_dir(dir: &Inode, name: &str) -> Option<u32> {
    scan_dir(dir).into_iter().find(|e| e.name == name).map(|e| e.ino)
}

// ── Path resolution ───────────────────────────────────────────────────────────

/// Resolve `path` to `(inode_number, Inode)`. Follows symlinks.
pub fn resolve_path(path: &str) -> Option<(u32, Inode)> {
    resolve_impl(0, String::from(path), 0)
}

fn resolve_impl(start_ino: u32, path: String, hops: usize) -> Option<(u32, Inode)> {
    if hops > MAX_SYMLINK_HOPS { return None; }

    let (mut ino, mut inode) = if path.starts_with('/') {
        (0u32, read_inode(0)?)
    } else {
        (start_ino, read_inode(start_ino)?)
    };

    let components: Vec<String> = path
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    let mut i = 0;
    while i < components.len() {
        let part = &components[i];
        i += 1;

        if inode.flags & INODE_DIR == 0 { return None; }

        let child_ino = lookup_in_dir(&inode, part)?;
        let child     = read_inode(child_ino)?;

        if child.flags & INODE_SYMLINK != 0 {
            let target_bytes = read_symlink_target(&child)?;
            let target       = core::str::from_utf8(&target_bytes).ok()?;

            // Append remaining components to the symlink target.
            let mut new_path = String::from(target);
            for j in i..components.len() {
                new_path.push('/');
                new_path.push_str(&components[j]);
            }

            let base_ino = if target.starts_with('/') { 0 } else { ino };
            return resolve_impl(base_ino, new_path, hops + 1);
        }

        ino   = child_ino;
        inode = child;
    }
    Some((ino, inode))
}

// ── Stat ──────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Default)]
pub struct Stat {
    pub size:   u64,
    pub flags:  u32,
    pub mode:   u16,
    pub uid:    u32,
    pub gid:    u32,
    pub nlink:  u32,
    pub mtime:  u64,
    pub ctime:  u64,
}

impl From<&Inode> for Stat {
    fn from(n: &Inode) -> Self {
        Stat { size: n.size, flags: n.flags, mode: n.mode, uid: n.uid,
               gid: n.gid, nlink: n.nlink, mtime: n.mtime, ctime: n.ctime }
    }
}

// ── Open-file table ───────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct OpenFile {
    ino_num: u32,
    inode:   Inode,
    offset:  u64,
}

struct RfsState {
    mounted:      bool,
    total_blocks: u32,
    fds:          [Option<OpenFile>; MAX_FDS],
}

static STATE: SpinLock<RfsState> = SpinLock::new(RfsState {
    mounted:      false,
    total_blocks: 0,
    fds:          [None; MAX_FDS],
});

// ── Mount ─────────────────────────────────────────────────────────────────────

/// Validate the superblock and mark the filesystem as mounted.
pub fn init() -> bool {
    if !virtio_blk::is_present() { return false; }

    let buf = match read_block(0) {
        Some(b) => b,
        None    => return false,
    };
    if &buf[0..8] != MAGIC { return false; }

    let total_blocks = get_u32(&buf, 16);
    let mut st       = STATE.lock();
    st.mounted       = true;
    st.total_blocks  = total_blocks;
    true
}

fn is_mounted() -> bool {
    STATE.lock().mounted
}

// ── VFS operations ────────────────────────────────────────────────────────────

/// Open a regular file by path. Returns fd (≥ 0) or a negative error code.
pub fn open(path: &[u8]) -> i64 {
    if !is_mounted() { return ENOMNT; }

    let path_str = match core::str::from_utf8(path) {
        Ok(s)  => s,
        Err(_) => return EINVAL,
    };
    let (ino_num, inode) = match resolve_path(path_str) {
        Some(r) => r,
        None    => return ENOENT,
    };
    if inode.flags & INODE_DIR != 0 { return EISDIR; }

    let mut st = STATE.lock();
    for (fd, slot) in st.fds.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(OpenFile { ino_num, inode, offset: 0 });
            return fd as i64;
        }
    }
    EMFILE
}

/// Read up to `buf.len()` bytes from `fd` at its current offset.
pub fn read(fd: u64, buf: &mut [u8]) -> i64 {
    if !is_mounted() { return ENOMNT; }
    if fd as usize >= MAX_FDS { return EBADF; }

    let mut st = STATE.lock();
    let of = match st.fds[fd as usize].as_mut() {
        Some(f) => f,
        None    => return EBADF,
    };
    let inode  = of.inode;
    let offset = of.offset;
    let n      = read_file_data(&inode, offset, buf);
    of.offset += n as u64;
    n as i64
}

/// Release `fd`.
pub fn close(fd: u64) -> i64 {
    if fd as usize >= MAX_FDS { return EBADF; }
    let mut st = STATE.lock();
    if st.fds[fd as usize].is_none() { return EBADF; }
    st.fds[fd as usize] = None;
    0
}

/// Fill `out` with stat info for `path`. Returns `true` on success.
pub fn stat_path(path: &[u8], out: &mut Stat) -> bool {
    if !is_mounted() { return false; }
    let path_str = match core::str::from_utf8(path) {
        Ok(s)  => s,
        Err(_) => return false,
    };
    match resolve_path(path_str) {
        Some((_, inode)) => { *out = Stat::from(&inode); true }
        None             => false,
    }
}

/// Read an entire file by path into a `Vec<u8>`. Returns `None` on error.
/// Capped at 32 MiB to guard against runaway allocations.
pub fn load_file(path: &str) -> Option<Vec<u8>> {
    let (_, inode) = resolve_path(path)?;
    if inode.flags & INODE_DIR != 0 { return None; }
    let size = inode.size as usize;
    if size == 0 || size > 32 * 1024 * 1024 { return None; }
    let mut buf = alloc::vec![0u8; size];
    let n = read_file_data(&inode, 0, &mut buf);
    if n < size { return None; }
    Some(buf)
}

/// Return directory entries for `path`, or `None` on error.
pub fn readdir_path(path: &[u8]) -> Option<Vec<DirEntry>> {
    if !is_mounted() { return None; }
    let path_str = match core::str::from_utf8(path) {
        Ok(s)  => s,
        Err(_) => return None,
    };
    let (_, inode) = resolve_path(path_str)?;
    if inode.flags & INODE_DIR == 0 { return None; }
    Some(scan_dir(&inode))
}
