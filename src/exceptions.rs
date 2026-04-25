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
/// If the fault originated in ring-3 (CPL=3), logs the fault address and
/// terminates only the faulting task — the kernel and all other tasks keep
/// running.  A kernel-mode #PF is unrecoverable and halts the CPU.
fn page_fault_handler(frame: &ExceptionFrame) -> ! {
    let cr2: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) cr2, options(nostack, nomem));
    }
    // Error code bits: P=present, W=write, U=user, R=reserved, I=ifetch.
    let p = frame.error_code & 1 != 0;
    let w = frame.error_code & 2 != 0;
    let u = frame.error_code & 4 != 0;
    kprintln!(
        "[#PF] faulting_va={:#x}  error={:#x}  rip={:#x}  {} {} {}",
        cr2, frame.error_code, frame.rip,
        if p { "protection-violation" } else { "not-present" },
        if w { "write" }               else { "read" },
        if u { "user"  }               else { "kernel" },
    );

    if frame.cs & 3 == 3 {
        // Ring-3 fault: CPU also pushed RSP and SS above the iret frame.
        let frame_ptr = frame as *const ExceptionFrame as *const u64;
        let struct_u64s = core::mem::size_of::<ExceptionFrame>() / 8;
        let user_rsp = unsafe { *frame_ptr.add(struct_u64s) };
        let user_ss  = unsafe { *frame_ptr.add(struct_u64s + 1) };
        kprintln!("[#PF] user RSP={:#x}  SS={:#x}", user_rsp, user_ss);
        // Terminate only the offending task.
        let tid = crate::task::current_task_id();
        kprintln!("[#PF] user task {} killed", tid);
        crate::task::task_exit();
    }

    // Kernel-mode page fault — unrecoverable.
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

    if f.cs & 3 == 3 {
        // Ring-3 exception: kill the task, keep the kernel alive.
        let tid = crate::task::current_task_id();
        kprintln!("[exception] user task {} killed (vec={:#x})", tid, f.vector);
        crate::task::task_exit();
    }

    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
