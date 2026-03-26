use crate::kprintln;

/// CPU exception handling.
///
/// `exception_handler` is called by the common ISR dispatcher in isr_stubs.s
/// with a pointer to the full saved CPU state on the stack.

/// Saved register state at exception entry.
///
/// Layout must exactly match the push sequence in `exception_common`
/// (isr_stubs.s).  Fields are ordered from lowest stack address (last push /
/// at RSP when exception_handler is entered) to highest (CPU-pushed frame).
#[repr(C)]
pub struct ExceptionFrame {
    // Saved by exception_common (pushed last → lowest address)
    pub r15: u64, pub r14: u64, pub r13: u64, pub r12: u64,
    pub r11: u64, pub r10: u64, pub r9:  u64, pub r8:  u64,
    pub rbp: u64, pub rdi: u64, pub rsi: u64,
    pub rdx: u64, pub rcx: u64, pub rbx: u64, pub rax: u64,
    // Pushed by the ISR stub
    pub vector:     u64,
    pub error_code: u64,
    // Pushed by the CPU on exception entry (same-privilege: no rsp/ss)
    pub rip:    u64,
    pub cs:     u64,
    pub rflags: u64,
}

/// Page fault handler (vector 14).
///
/// Reads CR2 (the faulting virtual address) and logs a diagnostic before
/// halting.  Step 5 wires this as a fatal handler; the VMM in a later step
/// can replace it with demand-paging or guard-page logic.
fn page_fault_handler(frame: &ExceptionFrame) {
    let cr2: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) cr2, options(nostack, nomem));
    }
    kprintln!(
        "[#PF] faulting_va={:#x}  error={:#x}  rip={:#x}",
        cr2, frame.error_code, frame.rip,
    );
    // Error code bits: P=present, W=write, U=user, R=reserved, I=ifetch.
    let p = frame.error_code & 1 != 0;
    let w = frame.error_code & 2 != 0;
    let u = frame.error_code & 4 != 0;
    kprintln!(
        "[#PF] {} {} {}",
        if p { "protection-violation" } else { "not-present" },
        if w { "write" }               else { "read" },
        if u { "user"  }               else { "kernel" },
    );
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

/// Common exception entry point, called from `exception_common` in isr_stubs.s.
#[unsafe(no_mangle)]
pub extern "C" fn exception_handler(frame: *const ExceptionFrame) {
    let f = unsafe { &*frame };

    if f.vector == 14 {
        page_fault_handler(f);
    }

    kprintln!(
        "[EXCEPTION] vec={:#x}  err={:#x}  rip={:#x}  cs={:#x}  rflags={:#x}",
        f.vector, f.error_code, f.rip, f.cs, f.rflags
    );
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
