/// Syscall interface — `syscall`/`sysretq` entry stub, dispatch, and
/// `enter_userspace`.
///
/// ## Syscall ABI
///
/// | Register | Role on entry          | Role on return |
/// |----------|------------------------|----------------|
/// | RAX      | syscall number         | return value   |
/// | RDI      | argument 1             |                |
/// | RSI      | argument 2             |                |
/// | RDX      | argument 3             |                |
/// | R10      | argument 4 (RCX clobbered by `syscall`) | |
/// | R8       | argument 5             |                |
/// | R9       | argument 6             |                |
/// | RCX      | user RIP (saved by CPU)|                |
/// | R11      | user RFLAGS (saved by CPU) |            |
///
/// ## Stack switch
///
/// The `syscall` instruction does not switch stacks.  `syscall_entry` saves
/// the user RSP in `SYSCALL_USER_RSP` and loads the kernel RSP from
/// `SYSCALL_KERN_RSP`.  `enter_userspace` sets both of these (plus
/// `tss::RSP0`) to the current task's kernel-stack top before `iretq`.
///
/// ## Syscall numbers
///
/// | Nr | Name            |
/// |----|-----------------|
/// |  0 | SYS_YIELD       |
/// |  1 | SYS_TASK_EXIT   |
/// |  2 | SYS_MMAP        |
/// |  3 | SYS_MUNMAP      |
/// |  4 | SYS_CAP_GRANT   |
/// |  5 | SYS_CAP_REVOKE  |
/// |  6 | SYS_IPC_SEND    |
/// |  7 | SYS_IPC_RECV    |
/// |  8 | SYS_IPC_CREATE  |
/// |  9 | SYS_ROLLBACK    |
/// | 10 | SYS_EXEC        |
/// | 11 | SYS_LOG         |
/// | 12 | SYS_IPC_SEND_CAP |
/// | 13 | SYS_IPC_RECV_CAP |
/// | 14 | SYS_SERIAL_READ  |
/// | 15 | SYS_TIME         |
/// | 16 | SYS_TASK_STATUS  |

use core::arch::global_asm;

// ── Syscall numbers ───────────────────────────────────────────────────────────

pub const SYS_YIELD:      u64 = 0;
pub const SYS_TASK_EXIT:  u64 = 1;
pub const SYS_MMAP:       u64 = 2;
pub const SYS_MUNMAP:     u64 = 3;
pub const SYS_CAP_GRANT:  u64 = 4;
pub const SYS_CAP_REVOKE: u64 = 5;
pub const SYS_IPC_SEND:   u64 = 6;
pub const SYS_IPC_RECV:   u64 = 7;
pub const SYS_IPC_CREATE: u64 = 8;
/// Privileged system reset.  Requires `CapKind::Rollback`.  Granted only to `lythd`.
pub const SYS_ROLLBACK:   u64 = 9;
/// Exec a new userspace process from an ELF blob in user memory.
pub const SYS_EXEC:       u64 = 10;
/// Write a UTF-8 string to the kernel serial console.  Debug aid only.
pub const SYS_LOG:        u64 = 11;
/// Send a message **and** transfer a capability over an IPC endpoint.
/// a1=ipc_cap_handle, a2=msg_ptr, a3=msg_len, a4=cap_handle_to_send
pub const SYS_IPC_SEND_CAP: u64 = 12;
/// Receive a message **and** accept any in-flight capability from an endpoint.
/// a1=ipc_cap_handle, a2=buf_ptr, a3=buf_len, a4=out_handle_ptr (user *mut u64)
/// Returns bytes received; writes new CapHandle to *out_handle_ptr (u64::MAX if none).
pub const SYS_IPC_RECV_CAP: u64 = 13;
/// Read bytes from the COM1 serial port into a user buffer.
/// a1=buf_ptr (user VA), a2=buf_len.
/// Blocks (yielding the CPU) until at least one byte is available, then
/// reads as many bytes as are ready (up to buf_len).  Returns bytes read.
pub const SYS_SERIAL_READ:  u64 = 14;
/// Return milliseconds elapsed since kernel boot (APIC tick counter).
/// No arguments.  Always succeeds; return value is a `u64` millisecond count.
pub const SYS_TIME:         u64 = 15;
/// Return the liveness status of a task by ID.
/// a1 = TaskId.
/// Returns: 0 = not found / dead, 1 = running or ready, 2 = blocked.
pub const SYS_TASK_STATUS:  u64 = 16;

// ── Error sentinel ────────────────────────────────────────────────────────────

/// Returned in RAX for unknown or unimplemented syscalls (analogous to ENOSYS).
pub const ENOSYS:  u64 = (-1i64) as u64;
/// Invalid or stale capability handle.
pub const ENOCAP:  u64 = (-2i64) as u64;
/// Insufficient capability rights for the requested operation.
pub const ENOPERM: u64 = (-3i64) as u64;
/// Invalid argument (e.g. target task not found, self-grant).
pub const EINVAL:  u64 = (-4i64) as u64;

// ── MSR addresses ─────────────────────────────────────────────────────────────

const IA32_EFER:  u32 = 0xC000_0080;
const IA32_STAR:  u32 = 0xC000_0081;
const IA32_LSTAR: u32 = 0xC000_0082;
const IA32_FMASK: u32 = 0xC000_0084;

// ── Per-CPU RSP save area ─────────────────────────────────────────────────────

/// Kernel RSP loaded on `syscall` entry.  Set by `enter_userspace`.
#[unsafe(no_mangle)]
pub static mut SYSCALL_KERN_RSP: u64 = 0;

/// User RSP saved on `syscall` entry, restored on `sysretq`.
#[unsafe(no_mangle)]
pub static mut SYSCALL_USER_RSP: u64 = 0;

// ── Assembly stubs ────────────────────────────────────────────────────────────
//
// Frame layout on the kernel stack after all pushes in syscall_entry
// (lower addresses first, [rsp+0] = r15):
//
//   [rsp +  0]  r15
//   [rsp +  8]  r14
//   [rsp + 16]  r13
//   [rsp + 24]  r12
//   [rsp + 32]  rbx
//   [rsp + 40]  rbp
//   [rsp + 48]  r11  — user RFLAGS (needed by sysretq)
//   [rsp + 56]  rcx  — user RIP    (needed by sysretq)
//   [rsp + 64]  rax  — syscall number
//   [rsp + 72]  rdi  — a1
//   [rsp + 80]  rsi  — a2
//   [rsp + 88]  rdx  — a3
//   [rsp + 96]  r10  — a4
//   [rsp +104]  r8   — a5
//   [rsp +112]  r9   — a6

global_asm!(r#"
.section .text

// ─────────────────────────────────────────────────────────────────────────────
// syscall_entry — LSTAR target.
//
// On entry: RAX=nr, RDI-R9=args, RCX=user RIP, R11=user RFLAGS, IF=0.
// ─────────────────────────────────────────────────────────────────────────────
.global syscall_entry
.type   syscall_entry, @function
syscall_entry:
    // Save user RSP to the global temporarily, then switch to the kernel stack.
    movq   %rsp, SYSCALL_USER_RSP(%rip)
    movq   SYSCALL_KERN_RSP(%rip), %rsp

    // Push user RSP FIRST (highest address on kernel stack, before the rest of
    // the frame).  This stores it per-task so that if yield_task() runs while
    // we are inside a syscall, another task's syscall cannot overwrite it via
    // the global SYSCALL_USER_RSP.
    pushq  SYSCALL_USER_RSP(%rip)   // saved user RSP (above SyscallFrame)

    pushq  %r9
    pushq  %r8
    pushq  %r10
    pushq  %rdx
    pushq  %rsi
    pushq  %rdi
    pushq  %rax      // nr
    pushq  %rcx      // user rip  (for sysretq)
    pushq  %r11      // user rflags (for sysretq)
    pushq  %rbp
    pushq  %rbx
    pushq  %r12
    pushq  %r13
    pushq  %r14
    pushq  %r15

    movq   %rsp, %rdi          // arg0: *mut SyscallFrame (r15 at lowest address)
    call   syscall_dispatch    // returns u64 in rax

    // Restore callee-saved regs without clobbering rax (return value).
    popq   %r15
    popq   %r14
    popq   %r13
    popq   %r12
    popq   %rbx
    popq   %rbp
    popq   %r11    // user rflags → R11 (used by sysretq)
    popq   %rcx    // user rip   → RCX (used by sysretq)

    // Restore user argument registers so they are preserved across syscalls.
    // Push order was: r9, r8, r10, rdx, rsi, rdi, rax(nr).
    // Skip nr (rax), then pop in reverse push order.
    addq   $8, %rsp            // skip nr (syscall number was in rax; we use rax for return value)
    popq   %rdi
    popq   %rsi
    popq   %rdx
    popq   %r10
    popq   %r8
    popq   %r9

    // Restore user RSP from the kernel stack (not the global — another task
    // may have overwritten SYSCALL_USER_RSP while we were inside yield_task).
    popq   %rsp
    sysretq

// ─────────────────────────────────────────────────────────────────────────────
// enter_userspace_asm(entry: u64, stack: u64) -> !
//
// rdi = user RIP,  rsi = user RSP
// Loads user segment registers, builds an iretq frame, and jumps to ring 3.
// ─────────────────────────────────────────────────────────────────────────────
.global enter_userspace_asm
.type   enter_userspace_asm, @function
enter_userspace_asm:
    // Set user data selector in DS/ES/FS/GS before crossing the ring boundary.
    movw   $0x1B, %ax   // USER_DATA_SEL | RPL=3  (0x18 | 3)
    movw   %ax,  %ds
    movw   %ax,  %es
    movw   %ax,  %fs
    movw   %ax,  %gs

    // iretq frame: SS | RSP | RFLAGS | CS | RIP  (pushed high → low)
    pushq  $0x1B          // SS:  user data | RPL=3
    pushq  %rsi           // RSP: user stack
    pushfq
    orq    $(1 << 9), (%rsp)   // ensure IF=1 in user RFLAGS
    pushq  $0x23          // CS:  user code | RPL=3  (0x20 | 3)
    pushq  %rdi           // RIP: user entry point
    iretq
"#, options(att_syntax));

unsafe extern "C" {
    fn syscall_entry();
    fn enter_userspace_asm(entry: u64, stack: u64) -> !;
}

// ── Syscall frame ─────────────────────────────────────────────────────────────

/// Register state pushed onto the kernel stack by `syscall_entry`.
///
/// Layout matches the push sequence in the assembly stub (r15 at the lowest
/// address, r9 at the highest).
#[repr(C)]
pub struct SyscallFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub r11: u64,   // user RFLAGS
    pub rcx: u64,   // user RIP
    pub nr:  u64,   // syscall number (RAX on entry)
    pub a1:  u64,   // RDI
    pub a2:  u64,   // RSI
    pub a3:  u64,   // RDX
    pub a4:  u64,   // R10
    pub a5:  u64,   // R8
    pub a6:  u64,   // R9
}

// ── SMAP state ────────────────────────────────────────────────────────────────

/// Set to `true` by `init()` when SMAP is detected and CR4.SMAP is enabled.
/// Consulted by `with_user_access` to gate STAC/CLAC emission — those
/// instructions are `#UD` on CPUs that don't advertise SMAP support.
static SMAP_ENABLED: core::sync::atomic::AtomicBool =
    core::sync::atomic::AtomicBool::new(false);

/// Execute `f` inside a SMAP-safe window.
///
/// If SMAP is active this issues `stac` (sets AC, allowing kernel access to
/// user pages) before calling `f` and `clac` (clears AC) afterwards.  If
/// SMAP is not active the call is a direct passthrough with no overhead.
///
/// # Safety
/// `f` must only touch user memory that has already been validated by
/// `valid_user_range`; the window must be as narrow as possible.
#[inline]
unsafe fn with_user_access<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    if SMAP_ENABLED.load(core::sync::atomic::Ordering::Relaxed) {
        unsafe { core::arch::asm!("stac", options(nostack, preserves_flags)); }
        let r = f();
        unsafe { core::arch::asm!("clac", options(nostack, preserves_flags)); }
        r
    } else {
        f()
    }
}

// ── User-pointer validation ───────────────────────────────────────────────────

/// Return `true` if `[ptr, ptr+len)` lies entirely in canonical user space.
///
/// Rejects: null pointers, arithmetic overflow, and addresses at or above the
/// user/kernel split (`0x0000_8000_0000_0000`).  Zero-length ranges are
/// accepted for any non-null pointer (no bytes are dereferenced).
#[inline]
fn valid_user_range(ptr: u64, len: u64) -> bool {
    if ptr == 0 { return false; }
    if len == 0 { return true; }
    match ptr.checked_add(len) {
        Some(end) => end <= 0x0000_8000_0000_0000,
        None      => false,
    }
}

// ── Syscall dispatch ──────────────────────────────────────────────────────────

/// Called by `syscall_entry` with a pointer to the kernel-stack frame.
/// The return value is placed in RAX before `sysretq`.
#[unsafe(no_mangle)]
pub extern "C" fn syscall_dispatch(frame: &mut SyscallFrame) -> u64 {
    match frame.nr {
        SYS_YIELD => {
            crate::task::yield_task();
            0
        }
        SYS_TASK_EXIT => {
            crate::task::task_exit();
        }
        SYS_MMAP => {
            // Require page-aligned virtual address.
            if frame.a1 & 0xFFF != 0 { return EINVAL; }
            // Reject the 0→1 GiB identity-map range (2 MiB huge pages — walk_or_create
            // would panic on the PS=1 entry) and all kernel-space addresses (above the
            // canonical user/kernel split) to prevent corrupting shared kernel page
            // table entries via the U/S propagation in walk_or_create.
            if frame.a1 < 0x4000_0000 || frame.a1 >= 0x0000_8000_0000_0000 { return EINVAL; }

            // Require a Memory capability with write access.
            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOPERM; }
            let table = unsafe { &*table_ptr };
            if !table.has_kind_with_rights(
                crate::cap::CapKind::Memory,
                crate::cap::CapRights::WRITE,
            ) {
                return ENOPERM;
            }

            let virt = crate::vmm::VirtAddr(frame.a1);

            // Reject double-map before touching page tables or the PMM.
            if !crate::task::vma_insert(frame.a1) { return EINVAL; }

            // Sanitize flags: keep only user-safe bits and force USER.
            let allowed = crate::vmm::PageFlags::PRESENT.0
                | crate::vmm::PageFlags::WRITABLE.0
                | crate::vmm::PageFlags::USER.0
                | crate::vmm::PageFlags::NX.0;
            let flags = crate::vmm::PageFlags(
                (frame.a3 & allowed) | crate::vmm::PageFlags::USER.0
            );

            // Allocate a fresh PMM frame (user cannot name a physical address).
            let Some(phys) = crate::pmm::alloc_frame() else {
                // Undo the VMA reservation we just made.
                crate::task::vma_remove(frame.a1);
                return EINVAL;
            };

            match crate::task::current_page_table() {
                Some(pml4) => crate::vmm::map_page_in(
                    crate::pmm::PhysAddr(pml4), virt, phys, flags
                ),
                None       => crate::vmm::map_page(virt, phys, flags),
            }
            0
        }
        SYS_MUNMAP => {
            // Require page-aligned virtual address in the user range.
            if frame.a1 & 0xFFF != 0 { return EINVAL; }
            if frame.a1 < 0x4000_0000 || frame.a1 >= 0x0000_8000_0000_0000 { return EINVAL; }

            // Reject unmaps for addresses this task never mapped.
            if !crate::task::vma_remove(frame.a1) { return EINVAL; }

            let virt = crate::vmm::VirtAddr(frame.a1);
            match crate::task::current_page_table() {
                Some(pml4) => {
                    let pml4_phys = crate::pmm::PhysAddr(pml4);
                    // Free the backing frame before clearing the PTE.
                    if let Some(phys) = crate::vmm::query_page_in(pml4_phys, virt) {
                        crate::pmm::free_frame(phys);
                    }
                    crate::vmm::unmap_page_in(pml4_phys, virt);
                    // Invalidate local TLB and shoot down all other CPUs.
                    unsafe {
                        core::arch::asm!(
                            "invlpg [{va}]",
                            va = in(reg) virt.as_u64(),
                            options(nostack, preserves_flags),
                        );
                    }
                    crate::apic::send_tlb_shootdown_ipi();
                }
                None => crate::vmm::unmap_page(virt),
            }
            0
        }
        SYS_CAP_GRANT => {
            let handle      = crate::cap::CapHandle(frame.a1);
            let target_id   = frame.a2;   // TaskId of recipient
            let rights_mask = crate::cap::CapRights(frame.a3 as u8);

            let current_id = crate::task::current_task_id();
            let from_ptr   = crate::task::cap_table_ptr(current_id);

            // Validate the handle first — ENOCAP takes priority over EINVAL.
            if from_ptr.is_null() { return ENOCAP; }
            let from = unsafe { &mut *from_ptr };
            if from.get(handle).is_err() { return ENOCAP; }

            let to_ptr = crate::task::cap_table_ptr(target_id);
            if to_ptr.is_null() || from_ptr == to_ptr {
                return EINVAL;
            }

            // SAFETY: from_ptr and to_ptr point to two *different* tasks' cap
            // tables; the single-threaded kernel guarantees no aliasing here.
            let to = unsafe { &mut *to_ptr };

            match crate::cap::cap_grant(from, handle, target_id, to, rights_mask) {
                Ok(new_handle) => new_handle.0,
                Err(crate::cap::CapError::NoGrant) => ENOPERM,
                Err(_) => ENOCAP,
            }
        }
        SYS_CAP_REVOKE => {
            let handle     = crate::cap::CapHandle(frame.a1);
            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);

            if table_ptr.is_null() { return ENOCAP; }
            let table = unsafe { &mut *table_ptr };

            match crate::cap::cap_cascade_revoke(table, handle, &mut |tid| {
                crate::task::cap_table_ptr(tid)
            }) {
                Ok(())                                    => 0,
                Err(crate::cap::CapError::NoRevoke)       => ENOPERM,
                Err(_)                                    => ENOCAP,
            }
        }
        SYS_IPC_CREATE => {
            // Allocate a ring-buffer page and register an IPC endpoint.
            // Returns a capability handle (CapHandle.0) to the caller.
            let endpoint_idx = match crate::ipc::create_endpoint() {
                Some(idx) => idx,
                None      => return EINVAL, // global endpoint cap reached
            };

            let obj = crate::cap::create_object(
                crate::cap::KernelObject::Ipc { endpoint_idx }
            ).expect("SYS_IPC_CREATE: KoTable OOM");

            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOCAP; }
            let table = unsafe { &mut *table_ptr };

            let handle = crate::cap::create_root_cap(
                table,
                crate::cap::CapKind::Ipc,
                crate::cap::CapRights::ALL,
                obj,
            );
            handle.0
        }
        SYS_IPC_SEND => {
            // a1 = CapHandle, a2 = msg_ptr (user VA), a3 = msg_len
            let handle  = crate::cap::CapHandle(frame.a1);
            let msg_len = (frame.a3 as usize).min(crate::ipc::MSG_SIZE);
            if !valid_user_range(frame.a2, msg_len as u64) { return EINVAL; }
            let msg_ptr = frame.a2 as *const u8;

            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOCAP; }
            let table = unsafe { &*table_ptr };

            let endpoint_idx = match table.get(handle) {
                Ok(c) if c.kind == crate::cap::CapKind::Ipc
                      && c.rights.has(crate::cap::CapRights::WRITE) => {
                    match crate::cap::get_object(c.object) {
                        Some(crate::cap::KernelObject::Ipc { endpoint_idx }) => *endpoint_idx,
                        _ => return ENOCAP,
                    }
                }
                Ok(_) => return ENOPERM,
                Err(_) => return ENOCAP,
            };

            let msg = unsafe { with_user_access(|| core::slice::from_raw_parts(msg_ptr, msg_len)) };
            crate::ipc::send(endpoint_idx, msg);
            0
        }
        SYS_IPC_RECV => {
            // a1 = CapHandle, a2 = buf_ptr (user VA), a3 = buf_len
            let handle  = crate::cap::CapHandle(frame.a1);
            let buf_len = (frame.a3 as usize).min(crate::ipc::MSG_SIZE);
            if !valid_user_range(frame.a2, buf_len as u64) { return EINVAL; }
            let buf_ptr = frame.a2 as *mut u8;

            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOCAP; }
            let table = unsafe { &*table_ptr };

            let endpoint_idx = match table.get(handle) {
                Ok(c) if c.kind == crate::cap::CapKind::Ipc
                      && c.rights.has(crate::cap::CapRights::READ) => {
                    match crate::cap::get_object(c.object) {
                        Some(crate::cap::KernelObject::Ipc { endpoint_idx }) => *endpoint_idx,
                        _ => return ENOCAP,
                    }
                }
                Ok(_) => return ENOPERM,
                Err(_) => return ENOCAP,
            };

            let mut buf = [0u8; crate::ipc::MSG_SIZE];
            let n = crate::ipc::recv(endpoint_idx, &mut buf);
            unsafe { with_user_access(|| {
                core::ptr::copy_nonoverlapping(buf.as_ptr(), buf_ptr, n.min(buf_len));
            }) };
            n as u64
        }
        SYS_ROLLBACK => {
            // Gate on the caller holding a CapKind::Rollback capability.
            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOPERM; }
            let table = unsafe { &*table_ptr };
            if !table.has_kind(crate::cap::CapKind::Rollback) { return ENOPERM; }

            // Privileged: halt the system for now.  lythd implements the actual
            // rollback policy; the kernel just verifies the capability and stops.
            crate::kprintln!("[rollback] triggered by task {} — halting", current_id);
            loop { unsafe { core::arch::asm!("hlt") }; }
        }
        SYS_EXEC => {
            // a1 = elf_ptr   (user VA, *const u8)
            // a2 = elf_len   (bytes)
            // a3 = caps_ptr  (user VA, *const u64 array of raw CapHandle values)
            // a4 = caps_len  (element count)
            // a5 = argv_ptr  (user VA, flat null-terminated strings: "arg0\0arg1\0…")
            // a6 = argv_bytes (total byte length of the argv buffer; 0 = no argv)
            extern crate alloc;
            use alloc::{string::String, vec::Vec};

            let elf_len    = frame.a2 as usize;
            let caps_len   = frame.a4 as usize;
            let argv_bytes = frame.a6 as usize;
            let caps_bytes = (caps_len as u64).saturating_mul(8);
            if !valid_user_range(frame.a1, elf_len as u64) { return EINVAL; }
            if caps_len > 0 && !valid_user_range(frame.a3, caps_bytes) { return EINVAL; }
            if argv_bytes > 0 && !valid_user_range(frame.a5, argv_bytes as u64) { return EINVAL; }
            if argv_bytes > 4000 { return EINVAL; } // fits in one stack page

            let elf_ptr  = frame.a1 as *const u8;
            let caps_ptr = frame.a3 as *const u64;

            // Copy ELF and caps into kernel-owned buffers while STAC is active.
            // The user pointers must NOT be dereferenced outside with_user_access
            // when SMAP is enabled.
            let mut elf_buf = alloc::vec![0u8; elf_len];
            unsafe { with_user_access(|| {
                core::ptr::copy_nonoverlapping(elf_ptr, elf_buf.as_mut_ptr(), elf_len);
            }) };

            let caps: Vec<crate::cap::CapHandle> = if caps_len == 0 {
                Vec::new()
            } else {
                let mut raw = alloc::vec![0u64; caps_len];
                unsafe { with_user_access(|| {
                    core::ptr::copy_nonoverlapping(caps_ptr, raw.as_mut_ptr(), caps_len);
                }) };
                raw.iter().map(|&h| crate::cap::CapHandle(h)).collect()
            };

            // Parse flat argv buffer: "arg0\0arg1\0…" → Vec<String>
            let argv_owned: Vec<String> = if argv_bytes == 0 || frame.a5 == 0 {
                Vec::new()
            } else {
                let mut argv_buf = alloc::vec![0u8; argv_bytes];
                unsafe { with_user_access(|| {
                    core::ptr::copy_nonoverlapping(
                        frame.a5 as *const u8,
                        argv_buf.as_mut_ptr(),
                        argv_bytes,
                    );
                }) };
                argv_buf.split(|&b| b == 0)
                   .filter(|s| !s.is_empty())
                   .filter_map(|s| core::str::from_utf8(s).ok().map(String::from))
                   .collect()
            };
            let argv_strs: Vec<&str> = argv_owned.iter().map(|s| s.as_str()).collect();

            match crate::elf::exec(&elf_buf, &caps, &argv_strs) {
                Ok(task_id) => task_id,
                Err(_)      => EINVAL,
            }
        }
        SYS_LOG => {
            // a1 = ptr (user VA, *const u8), a2 = len
            let len = frame.a2 as usize;
            if len == 0 { return 0; }
            if len > 4096 { return EINVAL; }
            if !valid_user_range(frame.a1, len as u64) { return EINVAL; }
            let bytes = unsafe { with_user_access(|| core::slice::from_raw_parts(frame.a1 as *const u8, len)) };
            if let Ok(s) = core::str::from_utf8(bytes) {
                crate::kprint!("{}", s);
            }
            0
        }
        SYS_IPC_SEND_CAP => {
            // a1=ipc_cap_handle, a2=msg_ptr, a3=msg_len, a4=cap_handle_to_send
            let ipc_handle      = crate::cap::CapHandle(frame.a1);
            let msg_len         = (frame.a3 as usize).min(crate::ipc::MSG_SIZE);
            if !valid_user_range(frame.a2, msg_len as u64) { return EINVAL; }
            let msg_ptr         = frame.a2 as *const u8;
            let send_cap_handle = crate::cap::CapHandle(frame.a4);

            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOCAP; }
            let table = unsafe { &mut *table_ptr };

            // Resolve the IPC endpoint (requires WRITE).
            let endpoint_idx = match table.get(ipc_handle) {
                Ok(c) if c.kind == crate::cap::CapKind::Ipc
                      && c.rights.has(crate::cap::CapRights::WRITE) => {
                    match crate::cap::get_object(c.object) {
                        Some(crate::cap::KernelObject::Ipc { endpoint_idx }) => *endpoint_idx,
                        _ => return ENOCAP,
                    }
                }
                Ok(_)  => return ENOPERM,
                Err(_) => return ENOCAP,
            };

            // Take (move) the capability out of the caller's table.
            let cap = match table.take(send_cap_handle) {
                Ok(c)  => c,
                Err(_) => return ENOCAP,
            };

            let msg = unsafe { with_user_access(|| core::slice::from_raw_parts(msg_ptr, msg_len)) };
            crate::ipc::send_cap(endpoint_idx, msg, cap);
            0
        }
        SYS_SERIAL_READ => {
            // a1 = buf_ptr (user VA, *mut u8), a2 = buf_len
            let buf_len = (frame.a2 as usize).min(4096);
            if buf_len == 0 { return 0; }
            if !valid_user_range(frame.a1, buf_len as u64) { return EINVAL; }
            let buf_ptr = frame.a1 as *mut u8;

            let mut tmp = [0u8; 4096];
            let mut n   = 0usize;

            // Block (yielding the CPU) until the first byte arrives, then
            // drain any additional bytes that are already in the FIFO.
            // try_read_byte() is destructive, so we store each byte immediately.
            loop {
                match crate::serial::SERIAL.lock().try_read_byte() {
                    Some(b) => { tmp[n] = b; n += 1; break; }
                    None    => crate::task::yield_task(),
                }
            }
            while n < buf_len {
                match crate::serial::SERIAL.lock().try_read_byte() {
                    Some(b) => { tmp[n] = b; n += 1; }
                    None    => break,
                }
            }

            unsafe { with_user_access(|| {
                core::ptr::copy_nonoverlapping(tmp.as_ptr(), buf_ptr, n);
            }) };
            n as u64
        }
        SYS_IPC_RECV_CAP => {
            // a1=ipc_cap_handle, a2=buf_ptr, a3=buf_len, a4=out_handle_ptr (*mut u64)
            let ipc_handle      = crate::cap::CapHandle(frame.a1);
            let buf_len         = (frame.a3 as usize).min(crate::ipc::MSG_SIZE);
            if !valid_user_range(frame.a2, buf_len as u64) { return EINVAL; }
            // out_handle_ptr is optional (0 = ignored); validate only if provided.
            if frame.a4 != 0 && !valid_user_range(frame.a4, 8) { return EINVAL; }
            let buf_ptr         = frame.a2 as *mut u8;
            let out_handle_ptr  = frame.a4 as *mut u64;

            let current_id = crate::task::current_task_id();
            let table_ptr  = crate::task::cap_table_ptr(current_id);
            if table_ptr.is_null() { return ENOCAP; }
            let table = unsafe { &*table_ptr };

            // Resolve the IPC endpoint (requires READ).
            let endpoint_idx = match table.get(ipc_handle) {
                Ok(c) if c.kind == crate::cap::CapKind::Ipc
                      && c.rights.has(crate::cap::CapRights::READ) => {
                    match crate::cap::get_object(c.object) {
                        Some(crate::cap::KernelObject::Ipc { endpoint_idx }) => *endpoint_idx,
                        _ => return ENOCAP,
                    }
                }
                Ok(_)  => return ENOPERM,
                Err(_) => return ENOCAP,
            };

            let mut buf = [0u8; crate::ipc::MSG_SIZE];
            let (n, maybe_cap) = crate::ipc::recv_cap(endpoint_idx, &mut buf);
            unsafe { with_user_access(|| {
                core::ptr::copy_nonoverlapping(buf.as_ptr(), buf_ptr, n.min(buf_len));
            }) };

            // Insert the received capability (if any) and write the handle.
            if out_handle_ptr as u64 != 0 {
                let handle_val = if let Some(cap) = maybe_cap {
                    let table_mut = unsafe { &mut *table_ptr };
                    table_mut.insert(cap).0
                } else {
                    u64::MAX
                };
                unsafe { with_user_access(|| out_handle_ptr.write(handle_val)) };
            }

            n as u64
        }
        SYS_TIME => {
            // Return milliseconds since boot — APIC tick counter (1 tick ≈ 1 ms).
            crate::apic::ticks()
        }
        SYS_TASK_STATUS => {
            // a1 = TaskId; returns 0=dead/missing, 1=running/ready, 2=blocked.
            crate::task::task_status_raw(frame.a1)
        }
        _ => ENOSYS,
    }
}

// ── MSR helpers ───────────────────────────────────────────────────────────────

#[inline]
unsafe fn wrmsr(msr: u32, value: u64) {
    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") value as u32,
            in("edx") (value >> 32) as u32,
            options(nostack, nomem),
        );
    }
}

#[inline]
unsafe fn rdmsr(msr: u32) -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") lo,
            out("edx") hi,
            options(nostack, nomem),
        );
    }
    (hi as u64) << 32 | lo as u64
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialise the syscall machinery.
///
/// 1. Enable `syscall`/`sysret` via `IA32_EFER.SCE` (bit 0).
/// 2. Configure `IA32_STAR`: kernel CS = 0x08, sysretq base = 0x10.
/// 3. Point `IA32_LSTAR` at `syscall_entry`.
/// 4. Set `IA32_FMASK` to clear IF on entry.
/// 5. Enable SMEP (CR4 bit 20) — prevents kernel-mode execution of user pages.
///
/// Must be called after `gdt::init()` and `vmm::init()`.
pub fn init() {
    unsafe {
        // 1. Set SCE in EFER
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | 1);

        // 2. STAR: bits[47:32] = kernel CS (0x08), bits[63:48] = sysretq base (0x10)
        //    sysretq: CS = 0x10 + 16 = 0x20 | RPL=3,  SS = 0x10 + 8 = 0x18 | RPL=3
        let star = (0x0010u64 << 48) | (0x0008u64 << 32);
        wrmsr(IA32_STAR, star);

        // 3. LSTAR = syscall_entry
        wrmsr(IA32_LSTAR, syscall_entry as *const () as u64);

        // 4. FMASK: clear IF (bit 9) on syscall entry
        wrmsr(IA32_FMASK, 1 << 9);

        // 5. Enable SMEP (CR4[20]) and SMAP (CR4[21]) if CPUID leaf 7 reports support.
        // CPUID.07H:EBX[7]  = SMEP,  EBX[20] = SMAP.
        // STAC/CLAC are only valid on CPUs that advertise SMAP; SMAP_ENABLED
        // gates their emission in the syscall handlers.
        {
            let ebx: u32;
            core::arch::asm!(
                "push rbx",
                "xor eax, eax",
                "xor ecx, ecx",
                "mov eax, 7",
                "cpuid",
                "mov {0:e}, ebx",
                "pop rbx",
                out(reg) ebx,
                lateout("eax") _,
                lateout("ecx") _,
                lateout("edx") _,
                options(nostack),
            );
            let mut cr4: u64;
            core::arch::asm!("mov {0}, cr4", out(reg) cr4, options(nostack, nomem));
            if (ebx >> 7) & 1 == 1  { cr4 |= 1u64 << 20; } // SMEP
            if (ebx >> 20) & 1 == 1 {
                cr4 |= 1u64 << 21;                           // SMAP
                SMAP_ENABLED.store(true, core::sync::atomic::Ordering::Relaxed);
            }
            core::arch::asm!("mov cr4, {0}", in(reg) cr4, options(nostack, nomem));
        }
    }
}

/// Enter ring-3 at `entry` with the user stack at `stack`.
///
/// Before the `iretq`, sets `SYSCALL_KERN_RSP` and `tss::RSP0` to the
/// current task's kernel-stack top so that subsequent syscalls and hardware
/// interrupts in ring 3 land on the correct kernel stack.  Never returns.
pub fn enter_userspace(entry: crate::vmm::VirtAddr, stack: crate::vmm::VirtAddr) -> ! {
    let kstack_top = crate::task::current_kernel_stack_top();
    if kstack_top != 0 {
        crate::tss::set_rsp0(kstack_top);
        unsafe { SYSCALL_KERN_RSP = kstack_top; }
    }
    unsafe { enter_userspace_asm(entry.as_u64(), stack.as_u64()) }
}
