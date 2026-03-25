/// x86_64 Task State Segment.
///
/// Only RSP0 is populated — the kernel-mode stack pointer loaded by the CPU
/// on every ring-3 → ring-0 privilege switch (hardware IRQs, exceptions, and
/// the `syscall`/`sysretq` path via `SYSCALL_KERN_RSP`).

use core::cell::UnsafeCell;

/// 64-bit TSS layout per Intel SDM Vol. 3A §7.7 (Table 7-11).
/// Total size: 104 bytes.
#[repr(C, packed)]
pub struct Tss {
    _reserved0: u32,       // 0
    pub rsp0:   u64,       // 4  — kernel stack for ring 3 → 0 transitions
    _rsp1:      u64,       // 12
    _rsp2:      u64,       // 20
    _reserved1: u64,       // 28
    _ist:       [u64; 7],  // 36 — IST1–7 (unused)
    _reserved2: u16,       // 92
    pub iopb:   u16,       // 94 — IOPB offset = sizeof(Tss) → no IOPB
}

const _: () = assert!(core::mem::size_of::<Tss>() == 96);

impl Tss {
    const fn zero() -> Self {
        Self {
            _reserved0: 0, rsp0: 0, _rsp1: 0, _rsp2: 0, _reserved1: 0,
            _ist: [0; 7], _reserved2: 0,
            iopb: 96, // offset past end of TSS → no IOPB
        }
    }
}

struct GlobalTss(UnsafeCell<Tss>);
// SAFETY: single-threaded kernel.
unsafe impl Sync for GlobalTss {}

static TSS: GlobalTss = GlobalTss(UnsafeCell::new(Tss::zero()));

/// Address of the TSS (for encoding the GDT descriptor).
#[inline]
pub fn tss_addr() -> u64 {
    TSS.0.get() as u64
}

/// Update RSP0 — the kernel stack used when a ring-3 → ring-0 transition
/// occurs.  Call this on every task switch to/from ring-3 tasks.
#[inline]
pub fn set_rsp0(rsp0: u64) {
    unsafe { (*TSS.0.get()).rsp0 = rsp0; }
}

/// Load the TSS into the Task Register.  Called once by `gdt::init()` after
/// the TSS descriptor has been written into the GDT.
pub fn load(selector: u16) {
    unsafe {
        core::arch::asm!(
            "ltr {0:x}",
            in(reg) selector,
            options(nostack, nomem),
        );
    }
}
