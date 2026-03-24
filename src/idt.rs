/// Interrupt Descriptor Table — 256 interrupt-gate entries.
///
/// Vectors 0–31 are wired to the ISR stubs defined in isr_stubs.s.
/// All other entries are left absent (not-present); they will be filled in
/// as hardware IRQ handlers are added in later steps.

/// A single 16-byte IDT entry (64-bit interrupt gate).
#[repr(C)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low:  u16, // handler[15:0]
    selector:    u16, // code segment selector
    ist:         u8,  // interrupt stack table index (0 = legacy stack)
    type_attr:   u8,  // P | DPL | 0 | type (0x8E = present, ring 0, int gate)
    offset_mid:  u16, // handler[31:16]
    offset_high: u32, // handler[63:32]
    _reserved:   u32,
}

impl IdtEntry {
    const fn absent() -> Self {
        Self {
            offset_low: 0, selector: 0, ist: 0, type_attr: 0,
            offset_mid: 0, offset_high: 0, _reserved: 0,
        }
    }

    fn interrupt(handler: u64) -> Self {
        Self {
            offset_low:  handler as u16,
            selector:    0x08,
            ist:         0,
            type_attr:   0x8E, // P=1, DPL=0, type=0b1110 (interrupt gate)
            offset_mid:  (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            _reserved:   0,
        }
    }
}

/// The 10-byte operand for `lidt` (limit:base).
#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base:  u64,
}

unsafe extern "C" {
    /// Array of ISR stub entry-point addresses, built in isr_stubs.s.
    static isr_stub_table: [u64; 32];
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::absent(); 256];

pub fn init() {
    // Wire vectors 0–31 to their ISR stubs.
    for i in 0..32_usize {
        let handler = unsafe { isr_stub_table[i] };
        unsafe { IDT[i] = IdtEntry::interrupt(handler) };
    }

    let ptr = IdtPtr {
        limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
        base:  &raw const IDT as u64,
    };
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) &raw const ptr, options(nostack, readonly));
    }
}
