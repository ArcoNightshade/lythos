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
/// | Nr | Name           |
/// |----|----------------|
/// |  0 | SYS_YIELD      |
/// |  1 | SYS_TASK_EXIT  |
/// |  2 | SYS_MMAP       |
/// |  3 | SYS_MUNMAP     |
/// |  4 | SYS_CAP_GRANT  |
/// |  5 | SYS_CAP_REVOKE |
/// |  6 | SYS_IPC_SEND   |
/// |  7 | SYS_IPC_RECV   |

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

// ── Error sentinel ────────────────────────────────────────────────────────────

/// Returned in RAX for unknown or unimplemented syscalls (analogous to ENOSYS).
pub const ENOSYS: u64 = u64::MAX;

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
    movq   %rsp, SYSCALL_USER_RSP(%rip)
    movq   SYSCALL_KERN_RSP(%rip), %rsp

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

    movq   %rsp, %rdi          // arg0: *mut SyscallFrame
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

    addq   $56, %rsp           // skip nr + a1..a6 (7 × 8 bytes)

    movq   SYSCALL_USER_RSP(%rip), %rsp
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
            let virt  = crate::vmm::VirtAddr(frame.a1);
            let phys  = crate::pmm::PhysAddr(frame.a2);
            let flags = crate::vmm::PageFlags(frame.a3);
            crate::vmm::map_page(virt, phys, flags);
            0
        }
        SYS_MUNMAP => {
            let virt = crate::vmm::VirtAddr(frame.a1);
            crate::vmm::unmap_page(virt);
            0
        }
        // IPC and capability syscalls: full implementation in Step 11.
        SYS_CAP_GRANT | SYS_CAP_REVOKE | SYS_IPC_SEND | SYS_IPC_RECV => ENOSYS,
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

        // 5. Enable SMEP (CR4 bit 20)
        let cr4: u64;
        core::arch::asm!(
            "mov {0}, cr4",
            out(reg) cr4,
            options(nostack, nomem),
        );
        core::arch::asm!(
            "mov cr4, {0}",
            in(reg) cr4 | (1u64 << 20),
            options(nostack, nomem),
        );
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
