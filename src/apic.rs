/// Local APIC and timer initialisation.
///
/// ## PIC disable
///
/// The legacy 8259 PIC is reinitialised (to remap its vectors away from CPU
/// exception space 0–31) and then fully masked.  All hardware IRQ delivery
/// goes through the Local APIC from this point on.
///
/// ## APIC MMIO
///
/// The Local APIC register file lives at a physical address reported by the
/// `IA32_APIC_BASE` MSR (default 0xFEE0_0000).  That physical address is
/// above the 1 GiB identity map, so we call `vmm::map_page` to create a
/// virtual mapping at `APIC_VIRT` before accessing any registers.
///
/// ## Timer calibration
///
/// The APIC timer frequency depends on the bus (or core-crystal) clock, which
/// varies by CPU and QEMU configuration.  We calibrate by counting APIC ticks
/// over a 10 ms PIT channel-2 reference window, then configure the timer for
/// a 1 ms periodic period.

use core::sync::atomic::{AtomicU64, Ordering};
use core::arch::global_asm;

// ── Virtual address for the APIC MMIO page ────────────────────────────────────
// Higher-half base (0xFFFF_8000_0000_0000) + default physical address.
const APIC_VIRT: u64 = 0xFFFF_8000_FEE0_0000;

// ── APIC register offsets (byte offsets into the MMIO page) ──────────────────
const REG_EOI:   usize = 0x0B0;
const REG_SVR:   usize = 0x0F0; // Spurious Interrupt Vector Register
const REG_TIMER: usize = 0x320; // LVT Timer
const REG_TICR:  usize = 0x380; // Timer Initial Count Register
const REG_TCCR:  usize = 0x390; // Timer Current Count Register (read-only)
const REG_TDCR:  usize = 0x3E0; // Timer Divide Configuration Register

// ── LVT Timer flags ───────────────────────────────────────────────────────────
const TIMER_PERIODIC: u32 = 1 << 17;
const TIMER_MASKED:   u32 = 1 << 16;

// ── Interrupt vectors ─────────────────────────────────────────────────────────
pub const VECTOR_TIMER:    u8 = 32;
pub const VECTOR_SPURIOUS: u8 = 255;

// ── MSR ───────────────────────────────────────────────────────────────────────
const IA32_APIC_BASE: u32 = 0x1B;

// ── Global tick counter ───────────────────────────────────────────────────────
static TICK_COUNT: AtomicU64 = AtomicU64::new(0);

// ── ISR assembly stubs ────────────────────────────────────────────────────────
//
// timer_isr_stub (vector 32):
//   Saves all caller-saved registers, calls timer_interrupt_handler(), restores,
//   iretq.  Callee-saved registers (rbp, rbx, r12-r15) are handled by
//   switch_context inside yield_task if a context switch occurs.
//
// spurious_isr_stub (vector 255):
//   Intel SDM §10.9: spurious interrupts must NOT be acknowledged (no EOI).
//   Just iretq.
global_asm!(r#"
.section .text

.global timer_isr_stub
.type   timer_isr_stub, @function
timer_isr_stub:
    pushq  %rax
    pushq  %rcx
    pushq  %rdx
    pushq  %rsi
    pushq  %rdi
    pushq  %r8
    pushq  %r9
    pushq  %r10
    pushq  %r11
    call   timer_interrupt_handler
    popq   %r11
    popq   %r10
    popq   %r9
    popq   %r8
    popq   %rdi
    popq   %rsi
    popq   %rdx
    popq   %rcx
    popq   %rax
    iretq

.global spurious_isr_stub
.type   spurious_isr_stub, @function
spurious_isr_stub:
    iretq
"#, options(att_syntax));

unsafe extern "C" {
    fn timer_isr_stub();
    fn spurious_isr_stub();
}

// ── MMIO helpers ──────────────────────────────────────────────────────────────

#[inline]
fn apic_read(offset: usize) -> u32 {
    unsafe {
        core::ptr::read_volatile((APIC_VIRT as usize + offset) as *const u32)
    }
}

#[inline]
fn apic_write(offset: usize, val: u32) {
    unsafe {
        core::ptr::write_volatile((APIC_VIRT as usize + offset) as *mut u32, val)
    }
}

// ── Port I/O helpers ──────────────────────────────────────────────────────────

#[inline]
unsafe fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!(
            "outb %al, %dx",
            in("dx") port, in("al") val,
            options(att_syntax, nostack, preserves_flags),
        );
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        core::arch::asm!(
            "inb %dx, %al",
            in("dx") port, out("al") val,
            options(att_syntax, nostack, preserves_flags),
        );
    }
    val
}

// ── Submodule: legacy PIC ─────────────────────────────────────────────────────

/// Reinitialise the 8259 PIC (remapping vectors to 0x20–0x2F so they don't
/// overlap CPU exceptions), then mask every IRQ line on both chips.
fn disable_pic() {
    unsafe {
        // ICW1: start initialisation sequence, cascade mode
        outb(0x20, 0x11); // PIC1 command
        outb(0xA0, 0x11); // PIC2 command

        // ICW2: vector offsets
        outb(0x21, 0x20); // PIC1: IRQ0-7  → vectors 0x20-0x27
        outb(0xA1, 0x28); // PIC2: IRQ8-15 → vectors 0x28-0x2F

        // ICW3: cascade wiring
        outb(0x21, 0x04); // PIC1: IR2 connected to PIC2
        outb(0xA1, 0x02); // PIC2: cascade identity 2

        // ICW4: 8086 mode
        outb(0x21, 0x01);
        outb(0xA1, 0x01);

        // OCW1: mask all IRQ lines
        outb(0x21, 0xFF);
        outb(0xA1, 0xFF);
    }
}

// ── Submodule: PIT calibration ────────────────────────────────────────────────

/// Block for approximately `ms` milliseconds using PIT channel 2.
///
/// Channel 2 is gated via bit 0 of port 0x61.  Its OUT pin (bit 5 of port
/// 0x61) goes high when the one-shot count reaches zero, giving us a
/// polled timer that doesn't disturb the IDT or require IRQs.
unsafe fn pit_wait_ms(ms: u32) {
    unsafe {
        // Briefly clear gate to reset channel 2 OUT, then configure.
        let saved = inb(0x61);
        outb(0x61, saved & !0x03); // gate off, speaker off

        // Channel 2, lo/hi byte, mode 0 (terminal count), binary: 0xB0
        outb(0x43, 0xB0);

        // count ≈ ms × 1193  (PIT runs at 1,193,182 Hz)
        let count = (ms * 1193) as u16;
        outb(0x42, count as u8);
        outb(0x42, (count >> 8) as u8);

        // Enable gate (bit 0 = 1), keep speaker off (bit 1 = 0).
        outb(0x61, (saved & !0x02) | 0x01);

        // Poll until OUT goes high (bit 5 of port 0x61).
        while inb(0x61) & 0x20 == 0 {}
    }
}

// ── Submodule: APIC base ──────────────────────────────────────────────────────

fn read_apic_base() -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") IA32_APIC_BASE,
            out("eax") lo,
            out("edx") hi,
            options(nostack, nomem),
        );
    }
    ((hi as u64) << 32 | lo as u64) & 0x000F_FFFF_FFFF_F000
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialise the Local APIC and start the preemption timer.
///
/// Must be called after `vmm::init()` (needs map_page) and `idt::init()`
/// (needs register_irq).
pub fn init() {
    disable_pic();

    let apic_phys = read_apic_base();

    // Map the APIC MMIO page into the kernel's virtual address space.
    // KERNEL_RW is sufficient under QEMU; on real hardware PWT|PCD (cache-
    // disable) bits should also be set to avoid speculative MMIO reads.
    crate::vmm::map_page(
        crate::vmm::VirtAddr(APIC_VIRT),
        crate::pmm::PhysAddr(apic_phys),
        crate::vmm::PageFlags::KERNEL_RW,
    );

    // Register ISR stubs in the IDT.
    crate::idt::register_irq(VECTOR_TIMER,    timer_isr_stub    as *const () as u64);
    crate::idt::register_irq(VECTOR_SPURIOUS, spurious_isr_stub as *const () as u64);

    // Enable the APIC: set SVR bit 8 (software enable) + spurious vector.
    apic_write(REG_SVR, (1 << 8) | VECTOR_SPURIOUS as u32);

    // ── Calibrate timer against PIT ───────────────────────────────────────
    // Set divide-by-16, start a masked one-shot count at max value, wait
    // 10 ms via PIT channel 2, then read back how many ticks elapsed.
    apic_write(REG_TDCR,  0x3);  // divide by 16
    apic_write(REG_TIMER, TIMER_MASKED | VECTOR_TIMER as u32);
    apic_write(REG_TICR,  0xFFFF_FFFF);

    unsafe { pit_wait_ms(10) };

    let remaining      = apic_read(REG_TCCR);
    let ticks_per_10ms = 0xFFFF_FFFFu32.wrapping_sub(remaining);
    let ticks_per_ms   = (ticks_per_10ms / 10).max(1); // guard against zero

    // ── Start periodic 1 ms timer ─────────────────────────────────────────
    apic_write(REG_TDCR,  0x3);
    apic_write(REG_TICR,  ticks_per_ms);
    apic_write(REG_TIMER, TIMER_PERIODIC | VECTOR_TIMER as u32); // unmasked
}

/// Signal end-of-interrupt to the Local APIC.
/// Must be called from every hardware IRQ handler before returning.
#[inline]
pub fn eoi() {
    apic_write(REG_EOI, 0);
}

/// Return the number of 1 ms timer ticks since `init()`.
#[inline]
pub fn ticks() -> u64 {
    TICK_COUNT.load(Ordering::Relaxed)
}

// ── IRQ handlers (called from assembly stubs) ─────────────────────────────────

/// Called by `timer_isr_stub` on every APIC timer tick (~1 ms).
#[unsafe(no_mangle)]
pub extern "C" fn timer_interrupt_handler() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    eoi(); // acknowledge before yielding so the next tick can be queued
    crate::task::yield_task();
}
