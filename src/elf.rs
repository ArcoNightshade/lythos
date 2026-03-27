/// ELF64 loader and `exec` — Step 12.
///
/// ## What this module does
///
/// `exec(elf_data, caps)` parses a static ELF64 binary, maps its `PT_LOAD`
/// segments into the current address space, allocates an 8 MiB user stack
/// (with a guard page at the bottom), writes an initial stack frame, spawns a
/// new kernel task that will enter ring-3 at the ELF entry point, and returns
/// the new task's ID.
///
/// ## Constraints
///
/// - Static ELF64 binaries only (`ET_EXEC`; dynamic linking deferred).
/// - Virtual addresses in `PT_LOAD` segments must be **above 1 GiB** — the
///   VMM identity-maps 0→1 GiB with 2 MiB huge pages and refuses to split them.
/// - All loaded segments share the kernel's page table (per-process isolation
///   deferred to a later step).
///
/// ## Stack layout at ring-3 entry
///
/// The initial stack frame follows the System V AMD64 ABI for `_start`:
///
/// ```text
/// rsp → [0]   argc  = 0
///        [8]   NULL  (end of argv)
///        [16]  NULL  (end of envp)
///        [24]  AT_PAGESZ (type = 6)
///        [32]  4096
///        [40]  AT_NULL (type = 0)
///        [48]  0
/// ```
///
/// `rsp` is 16-byte aligned at entry.

extern crate alloc;

use core::sync::atomic::{AtomicU64, Ordering};

use crate::cap::CapHandle;
use crate::pmm::PhysAddr;
use crate::task::TaskId;
use crate::vmm::{PageFlags, VirtAddr};

// ── ELF64 constants ───────────────────────────────────────────────────────────

const ELFMAG:      [u8; 4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64:  u8  = 2;
const ELFDATA2LSB: u8  = 1;
const ET_EXEC:     u16 = 2;
const EM_X86_64:   u16 = 62;
const PT_LOAD:     u32 = 1;
const PF_X:        u32 = 1;
const PF_W:        u32 = 2;

// ── ELF64 structs ─────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy)]
struct Elf64Ehdr {
    e_ident:     [u8; 16],
    e_type:      u16,
    e_machine:   u16,
    e_version:   u32,
    e_entry:     u64,
    e_phoff:     u64,
    e_shoff:     u64,
    e_flags:     u32,
    e_ehsize:    u16,
    e_phentsize: u16,
    e_phnum:     u16,
    e_shentsize: u16,
    e_shnum:     u16,
    e_shstrndx:  u16,
}

const ELF64_EHDR_SIZE: usize = core::mem::size_of::<Elf64Ehdr>();

#[repr(C)]
#[derive(Clone, Copy)]
struct Elf64Phdr {
    p_type:   u32,
    p_flags:  u32,
    p_offset: u64,
    p_vaddr:  u64,
    p_paddr:  u64,
    p_filesz: u64,
    p_memsz:  u64,
    p_align:  u64,
}

const ELF64_PHDR_SIZE: usize = core::mem::size_of::<Elf64Phdr>();

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum ElfError {
    TooSmall,
    BadMagic,
    Not64Bit,
    NotLittleEndian,
    UnsupportedType,
    UnsupportedMachine,
    BadProgramHeader,
    SegmentOutOfBounds,
}

// ── Stack constants ────────────────────────────────────────────────────────────

/// Base VA of the first user-mode stack slot.
/// Must be above 1 GiB (identity-mapped region) and below the kernel half.
const STACK_GUARD_VA:   u64   = 0x0000_7FFF_0000_0000;
/// 8 MiB = 2048 × 4 KiB pages of usable stack above the guard.
const USER_STACK_PAGES: usize = 2048;
/// Pages per stack slot: guard + usable + 1 gap = 2050.
const STACK_SLOT_PAGES: u64   = USER_STACK_PAGES as u64 + 2;

/// Monotonically increasing counter for stack slot allocation.
/// Each `alloc_user_stack` call claims one slot of `STACK_SLOT_PAGES` pages.
static NEXT_STACK_SLOT: AtomicU64 = AtomicU64::new(0);

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Read an `Elf64Ehdr` from `data` without assuming alignment.
fn read_ehdr(data: &[u8]) -> Result<Elf64Ehdr, ElfError> {
    if data.len() < ELF64_EHDR_SIZE { return Err(ElfError::TooSmall); }
    Ok(unsafe { (data.as_ptr() as *const Elf64Ehdr).read_unaligned() })
}

/// Read the `n`-th `Elf64Phdr` from `data` given base offset and entry size.
fn read_phdr(data: &[u8], phoff: usize, phentsize: usize, n: usize)
    -> Result<Elf64Phdr, ElfError>
{
    let off = phoff + n * phentsize;
    if off + ELF64_PHDR_SIZE > data.len() { return Err(ElfError::BadProgramHeader); }
    Ok(unsafe { (data.as_ptr().add(off) as *const Elf64Phdr).read_unaligned() })
}

/// Choose page flags for a `PT_LOAD` segment based on its `p_flags`.
fn segment_flags(p_flags: u32) -> PageFlags {
    let exec  = p_flags & PF_X != 0;
    let write = p_flags & PF_W != 0;
    match (exec, write) {
        (true,  false) => PageFlags::USER_RX,
        (false, true ) => PageFlags::USER_RW,
        // R/W/X: present + user + writable (no NX).
        (true,  true ) => PageFlags(
            PageFlags::PRESENT.0 | PageFlags::USER.0 | PageFlags::WRITABLE.0
        ),
        // R only: present + user + NX.
        (false, false) => PageFlags(
            PageFlags::PRESENT.0 | PageFlags::USER.0 | PageFlags::NX.0
        ),
    }
}

/// Load a single `PT_LOAD` segment into `user_pml4`.
///
/// All reads and writes go through the **physical** (identity-mapped) address
/// of each frame so that this function works while the kernel PML4 is the
/// active CR3 — the user PT is not yet loaded.  Page-aligned arithmetic is
/// used throughout to avoid spilling into unmapped adjacent pages.  If a page
/// was already mapped by an earlier overlapping segment the physical frame is
/// reused and only the flags are upgraded.
fn load_segment_into(data: &[u8], phdr: &Elf64Phdr, user_pml4: PhysAddr) -> Result<(), ElfError> {
    let file_off   = phdr.p_offset as usize;
    let file_size  = phdr.p_filesz as usize;
    let mem_size   = phdr.p_memsz  as usize;
    let vaddr_base = phdr.p_vaddr;
    let flags      = segment_flags(phdr.p_flags);

    if file_off + file_size > data.len() { return Err(ElfError::SegmentOutOfBounds); }
    if mem_size == 0 { return Ok(()); }

    let vaddr_end  = vaddr_base + mem_size as u64;
    let page_start = vaddr_base & !0xFFF;
    let page_end   = (vaddr_end + 0xFFF) & !0xFFF;
    let page_count = ((page_end - page_start) / 0x1000) as usize;

    for i in 0..page_count {
        let page_va = page_start + (i as u64) * 0x1000;

        // Resolve or allocate the physical frame for this page.
        let frame = if let Some(p) = crate::vmm::query_page_in(user_pml4, VirtAddr(page_va)) {
            crate::vmm::update_page_flags_in(user_pml4, VirtAddr(page_va), flags);
            p
        } else {
            let f = crate::pmm::alloc_frame()
                .expect("elf::load_segment_into: out of physical frames");
            crate::vmm::map_page_in(user_pml4, VirtAddr(page_va), f, flags);
            // Zero entirely through the identity map.
            unsafe { core::ptr::write_bytes(f.as_u64() as *mut u8, 0, 0x1000); }
            f
        };

        // Copy the file bytes for this page (if any) through the physical frame.
        let seg_file_end  = vaddr_base + file_size as u64;
        let copy_va_start = vaddr_base.max(page_va);
        let copy_va_end   = seg_file_end.min(page_va + 0x1000);

        if copy_va_start < copy_va_end {
            let frame_off    = (copy_va_start - page_va) as usize;
            let file_src_off = file_off + (copy_va_start - vaddr_base) as usize;
            let copy_len     = (copy_va_end - copy_va_start) as usize;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    data.as_ptr().add(file_src_off),
                    (frame.as_u64() as *mut u8).add(frame_off),
                    copy_len,
                );
            }
        }
    }
    Ok(())
}

/// Allocate and map the user stack for one task into `user_pml4`.
///
/// Slot layout (STACK_SLOT_PAGES pages wide):
///   slot_base + 0              — guard page (no USER bit → #PF on access)
///   slot_base + 4 KiB          — first usable stack page
///   slot_base + N × 4 KiB      — last usable stack page  (N = USER_STACK_PAGES)
///   slot_base + (N+1) × 4 KiB  — stack top (returned; gap page for next slot)
///
/// Returns `(stack_top_va, last_usable_page_phys)`.  The physical address is
/// needed by `write_initial_stack_frame` to write the ABI frame through the
/// identity map while the kernel PML4 is still active.
fn alloc_user_stack_into(user_pml4: PhysAddr) -> (VirtAddr, PhysAddr) {
    let slot      = NEXT_STACK_SLOT.fetch_add(1, Ordering::Relaxed);
    let slot_base = STACK_GUARD_VA + slot * STACK_SLOT_PAGES * 0x1000;

    // Guard page: kernel-only, NX.
    let guard_frame = crate::pmm::alloc_frame()
        .expect("elf::alloc_user_stack_into: out of frames for guard page");
    crate::vmm::map_page_in(
        user_pml4,
        VirtAddr(slot_base),
        guard_frame,
        PageFlags(PageFlags::PRESENT.0 | PageFlags::NX.0),
    );
    unsafe { core::ptr::write_bytes(guard_frame.as_u64() as *mut u8, 0, 0x1000); }

    // Usable stack pages.
    let mut last_frame = PhysAddr(0);
    for i in 1..=(USER_STACK_PAGES as u64) {
        let frame = crate::pmm::alloc_frame()
            .expect("elf::alloc_user_stack_into: out of frames for stack");
        crate::vmm::map_page_in(user_pml4, VirtAddr(slot_base + i * 0x1000), frame, PageFlags::USER_RW);
        unsafe { core::ptr::write_bytes(frame.as_u64() as *mut u8, 0, 0x1000); }
        if i == USER_STACK_PAGES as u64 { last_frame = frame; }
    }

    let stack_top = VirtAddr(slot_base + (USER_STACK_PAGES as u64 + 1) * 0x1000);
    (stack_top, last_frame)
}

/// Write the initial ABI stack frame below `stack_top` through `last_page_phys`.
///
/// The stack pages are mapped in the user PT, which is not the active CR3
/// during `exec`.  Writes go through the physical (identity-mapped) address
/// so no page-table switch is required.
///
/// Frame (lowest address first, rsp points to argc):
/// ```text
/// [rsp+0]   argc  = 0
/// [rsp+8]   NULL  (argv end)
/// [rsp+16]  NULL  (envp end)
/// [rsp+24]  6     (AT_PAGESZ type)
/// [rsp+32]  4096  (AT_PAGESZ value)
/// [rsp+40]  0     (AT_NULL type)
/// [rsp+48]  0     (AT_NULL value)
/// ```
fn write_initial_stack_frame(stack_top: VirtAddr, last_page_phys: PhysAddr) -> VirtAddr {
    let rsp         = (stack_top.as_u64() - 56) & !0xF;
    let page_offset = rsp & 0xFFF;
    let p           = (last_page_phys.as_u64() + page_offset) as *mut u64;
    unsafe {
        p.add(0).write(0);    // argc
        p.add(1).write(0);    // argv end (NULL)
        p.add(2).write(0);    // envp end (NULL)
        p.add(3).write(6);    // AT_PAGESZ type
        p.add(4).write(4096); // AT_PAGESZ value
        p.add(5).write(0);    // AT_NULL type
        p.add(6).write(0);    // AT_NULL value
    }
    VirtAddr(rsp)
}

// ── exec trampoline ───────────────────────────────────────────────────────────

/// Kernel-mode entry point for tasks created by `exec`.
///
/// Reads the user entry point and user stack top from the current task's
/// stored fields (set by `spawn_userspace_task`) and transfers control to
/// ring-3 via `iretq`.  Never returns.
fn exec_trampoline() -> ! {
    let (entry, stack) = crate::task::current_entry_and_stack();
    crate::syscall::enter_userspace(entry, stack);
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Load and execute a static ELF64 binary.
///
/// Steps performed:
/// 1. Parse the ELF header and validate magic / class / type.
/// 2. For each `PT_LOAD` segment: allocate frames, map, copy file data,
///    zero-fill BSS.
/// 3. Allocate an 8 MiB user stack with a guard page.
/// 4. Write the initial ABI stack frame.
/// 5. Inherit `caps` into the new task's capability table.
/// 6. Spawn a kernel task that will enter ring-3 at the ELF entry point.
///
/// Returns the `TaskId` of the newly created task.  The task is enqueued in
/// the scheduler but does not run until the caller yields.
pub fn exec(elf_data: &[u8], caps: &[CapHandle]) -> Result<TaskId, ElfError> {
    // ── 1. Parse header ───────────────────────────────────────────────────
    let ehdr = read_ehdr(elf_data)?;

    if ehdr.e_ident[..4] != ELFMAG           { return Err(ElfError::BadMagic); }
    if ehdr.e_ident[4]   != ELFCLASS64       { return Err(ElfError::Not64Bit); }
    if ehdr.e_ident[5]   != ELFDATA2LSB      { return Err(ElfError::NotLittleEndian); }
    if ehdr.e_type        != ET_EXEC         { return Err(ElfError::UnsupportedType); }
    if ehdr.e_machine     != EM_X86_64       { return Err(ElfError::UnsupportedMachine); }

    let phoff     = ehdr.e_phoff     as usize;
    let phentsize = ehdr.e_phentsize as usize;
    let phnum     = ehdr.e_phnum     as usize;

    // ── 2. Create per-process page table ─────────────────────────────────
    let user_pml4 = crate::vmm::create_user_page_table();

    // ── 3. Load PT_LOAD segments into the new page table ─────────────────
    for i in 0..phnum {
        let phdr = read_phdr(elf_data, phoff, phentsize, i)?;
        if phdr.p_type != PT_LOAD { continue; }
        load_segment_into(elf_data, &phdr, user_pml4)?;
    }

    // ── 4. User stack ─────────────────────────────────────────────────────
    let (stack_top, last_page_phys) = alloc_user_stack_into(user_pml4);

    // ── 5. Initial stack frame (through physical address) ─────────────────
    let initial_sp = write_initial_stack_frame(stack_top, last_page_phys);

    // ── 6. Spawn task with its own page table ─────────────────────────────
    let task_id = crate::task::spawn_userspace_task(
        VirtAddr(ehdr.e_entry),
        initial_sp,
        caps,
        exec_trampoline,
        user_pml4.as_u64(),
    );

    Ok(task_id)
}

// ── Smoke-test binary ─────────────────────────────────────────────────────────
//
// A hand-crafted ELF64 binary that calls SYS_TASK_EXIT (nr=1) and halts.
//
// Header layout:
//   [  0.. 63] ELF header
//   [ 64..119] PT_LOAD program header (p_vaddr=0x100000000, p_filesz/memsz=128)
//   [120..127] code: mov eax,1 ; syscall ; hlt
//
// Entry point: 0x0000_0001_0000_0078  (0x100000000 + 120)
//
// The load VA is chosen to be above the VMM's 0→1 GiB identity-mapped region.

/// The real lythd init process binary, compiled from RaptorOS/lythd.
///
/// Built with `cargo build --release` in the RaptorOS workspace targeting
/// `x86_64-raptoros`.  Entry point is `_start` via `userspace.ld`.
///
/// lythd:
///   1. Receives the BootInfo message on cap handle 2.
///   2. Prints system info via SYS_LOG.
///   3. Creates the service registry IPC endpoint.
///   4. Blocks in the supervisor loop waiting for service registrations.
pub static LYTHD_ELF: &[u8] =
    include_bytes!("../../RaptorOS/target/x86_64-raptoros/release/lythd");

pub static SMOKE_ELF: &[u8] = &[
    // ── ELF header (64 bytes) ─────────────────────────────────────────────
    0x7F, 0x45, 0x4C, 0x46,              // ELF magic
    0x02,                                 // EI_CLASS:   ELFCLASS64
    0x01,                                 // EI_DATA:    ELFDATA2LSB
    0x01,                                 // EI_VERSION: 1
    0x00,                                 // EI_OSABI:   System V
    0x00, 0x00, 0x00, 0x00,              // padding
    0x00, 0x00, 0x00, 0x00,              // padding
    0x02, 0x00,                           // e_type:      ET_EXEC
    0x3E, 0x00,                           // e_machine:   EM_X86_64
    0x01, 0x00, 0x00, 0x00,              // e_version:   1
    0x78, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // e_entry: 0x100000078
    0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // e_phoff: 64
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // e_shoff: 0
    0x00, 0x00, 0x00, 0x00,              // e_flags: 0
    0x40, 0x00,                           // e_ehsize:    64
    0x38, 0x00,                           // e_phentsize: 56
    0x01, 0x00,                           // e_phnum:     1
    0x40, 0x00,                           // e_shentsize: 64
    0x00, 0x00,                           // e_shnum:     0
    0x00, 0x00,                           // e_shstrndx:  0

    // ── PT_LOAD program header (56 bytes) ─────────────────────────────────
    0x01, 0x00, 0x00, 0x00,              // p_type:   PT_LOAD
    0x05, 0x00, 0x00, 0x00,              // p_flags:  PF_R | PF_X
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_offset: 0
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // p_vaddr:  0x100000000
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // p_paddr:  0x100000000
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_filesz: 128
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_memsz:  128
    0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_align:  0x1000

    // ── Code (8 bytes at file offset 120) ─────────────────────────────────
    0xB8, 0x01, 0x00, 0x00, 0x00,        // mov eax, 1   (SYS_TASK_EXIT)
    0x0F, 0x05,                           // syscall
    0xF4,                                 // hlt          (should not reach)
];

// ── Step 14 integration ELFs ──────────────────────────────────────────────────
//
// Two userspace tasks for the end-to-end IPC smoke test.
// They use a *shared* IPC capability at handle 0 (the only cap they inherit).
//
// Different p_vaddr values keep them from clobbering each other's code pages
// in the shared kernel page table.

/// Minimal IPC sender task (p_vaddr=0x200000000).
///
/// Assembly (entry at file offset 120 = VA 0x200000078):
/// ```asm
/// mov  eax, 6       ; SYS_IPC_SEND
/// xor  edi, edi     ; a1 = handle 0 (ipc_cap)
/// mov  rsi, rsp     ; a2 = buf (initial stack frame on rsp)
/// mov  edx, 64      ; a3 = len
/// syscall
/// mov  eax, 1       ; SYS_TASK_EXIT
/// syscall
/// hlt
/// ```
pub static IPC_SENDER_ELF: &[u8] = &[
    // ── ELF header (64 bytes) ─────────────────────────────────────────────
    0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x78, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,  // e_entry: 0x200000078
    0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // e_phoff: 64
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x40, 0x00, 0x38, 0x00, 0x01, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ── PT_LOAD (56 bytes) ────────────────────────────────────────────────
    0x01, 0x00, 0x00, 0x00,              // PT_LOAD
    0x05, 0x00, 0x00, 0x00,              // PF_R | PF_X
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,  // p_vaddr: 0x200000000
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_filesz: 145
    0x91, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_memsz:  145
    0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ── Code (25 bytes at file offset 120) ───────────────────────────────
    0xB8, 0x06, 0x00, 0x00, 0x00,        // mov  eax, 6   (SYS_IPC_SEND)
    0x31, 0xFF,                           // xor  edi, edi (handle = 0)
    0x48, 0x89, 0xE6,                    // mov  rsi, rsp
    0xBA, 0x40, 0x00, 0x00, 0x00,        // mov  edx, 64
    0x0F, 0x05,                           // syscall
    0xB8, 0x01, 0x00, 0x00, 0x00,        // mov  eax, 1   (SYS_TASK_EXIT)
    0x0F, 0x05,                           // syscall
    0xF4,                                 // hlt
];

/// Minimal IPC receiver task (p_vaddr=0x300000000).
///
/// Assembly (entry at file offset 120 = VA 0x300000078):
/// ```asm
/// sub  rsp, 72      ; room for recv buffer
/// mov  eax, 7       ; SYS_IPC_RECV
/// xor  edi, edi     ; a1 = handle 0 (ipc_cap)
/// mov  rsi, rsp     ; a2 = buf
/// mov  edx, 64      ; a3 = len
/// syscall
/// mov  eax, 1       ; SYS_TASK_EXIT
/// syscall
/// hlt
/// ```
pub static IPC_RECEIVER_ELF: &[u8] = &[
    // ── ELF header (64 bytes) ─────────────────────────────────────────────
    0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x78, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,  // e_entry: 0x300000078
    0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x40, 0x00, 0x38, 0x00, 0x01, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ── PT_LOAD (56 bytes) ────────────────────────────────────────────────
    0x01, 0x00, 0x00, 0x00,
    0x05, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,  // p_vaddr: 0x300000000
    0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
    0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_filesz: 149
    0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // p_memsz:  149
    0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // ── Code (29 bytes at file offset 120) ───────────────────────────────
    0x48, 0x83, 0xEC, 0x48,              // sub  rsp, 72
    0xB8, 0x07, 0x00, 0x00, 0x00,        // mov  eax, 7   (SYS_IPC_RECV)
    0x31, 0xFF,                           // xor  edi, edi (handle = 0)
    0x48, 0x89, 0xE6,                    // mov  rsi, rsp
    0xBA, 0x40, 0x00, 0x00, 0x00,        // mov  edx, 64
    0x0F, 0x05,                           // syscall
    0xB8, 0x01, 0x00, 0x00, 0x00,        // mov  eax, 1   (SYS_TASK_EXIT)
    0x0F, 0x05,                           // syscall
    0xF4,                                 // hlt
];
