/// Kernel Global Descriptor Table.
///
/// Three entries: null, 64-bit kernel code (ring 0), kernel data (ring 0).
/// Matches the descriptors used by the boot stub, but now Rust-owned so the
/// linker places them in .rodata and lgdt points here instead of boot.s.

use core::mem;

pub const KERNEL_CODE_SEL: u16 = 0x08;
pub const KERNEL_DATA_SEL: u16 = 0x10;

static GDT: [u64; 3] = [
    0x0000000000000000, // 0x00 — null descriptor
    0x00AF9A000000FFFF, // 0x08 — 64-bit code, ring 0 (L=1, D=0, P=1)
    0x00CF92000000FFFF, // 0x10 — data, ring 0
];

/// The 10-byte operand for `lgdt` (limit:base).
#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base:  u64,
}

unsafe extern "C" {
    /// Defined in isr_stubs.s: loads the GDT and reloads all segment regs.
    fn gdt_flush(ptr: *const GdtPtr);
}

pub fn init() {
    let ptr = GdtPtr {
        limit: (mem::size_of_val(&GDT) - 1) as u16,
        base:  GDT.as_ptr() as u64,
    };
    unsafe { gdt_flush(&raw const ptr); }
}
