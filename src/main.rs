#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::arch::global_asm;
use core::panic::PanicInfo;

pub mod apic;
pub mod cap;
pub mod heap;
mod exceptions;
mod gdt;
mod idt;
pub mod pmm;
pub mod serial;
pub mod syscall;
pub mod task;
pub mod tss;
pub mod vmm;

// Boot stub: Multiboot headers + 32-bit → 64-bit long-mode transition.
global_asm!(include_str!("arch/x86_64/boot.s"), options(att_syntax));

// ISR stubs for vectors 0–31, gdt_flush helper, isr_stub_table.
global_asm!(include_str!("arch/x86_64/isr_stubs.s"), options(att_syntax));

/// Kernel entry point — called by the boot stub in 64-bit long mode.
///
/// `mb_magic`: Multiboot magic (0x2BADB002 for MB1, 0x36D76289 for MB2).
/// `mb_info`:  Physical address of the Multiboot info structure.
#[unsafe(no_mangle)]
pub extern "C" fn kmain(mb_magic: u32, mb_info: u64) -> ! {
    serial::init();
    kprintln!("lythos kernel initializing...");

    gdt::init();
    kprintln!("[gdt] loaded");

    idt::init();
    kprintln!("[idt] loaded - exceptions active");

    // ── Physical memory manager ──────────────────────────────────────────
    pmm::init(mb_magic, mb_info);
    kprintln!(
        "[pmm] initialized — {} free frames ({} MiB)",
        pmm::free_frame_count(),
        pmm::free_frame_count() * 4 / 1024
    );

    // ── Smoke-test: alloc 1000 frames, free, re-alloc, verify same addrs ─
    let mut frames = [pmm::PhysAddr(0); 1000];
    for f in frames.iter_mut() {
        *f = pmm::alloc_frame().expect("pmm smoke-test: out of frames");
    }
    for &f in frames.iter().rev() {
        pmm::free_frame(f);
    }
    for (i, f) in frames.iter().enumerate() {
        let got = pmm::alloc_frame().expect("pmm smoke-test: out of frames on re-alloc");
        assert_eq!(got, *f, "pmm smoke-test: frame mismatch at index {}", i);
    }
    // Return the 1000 frames so they don't pollute later allocations.
    for &f in frames.iter().rev() {
        pmm::free_frame(f);
    }
    kprintln!("[pmm] smoke-test passed");

    // ── Virtual memory manager ────────────────────────────────────────────
    vmm::init();
    kprintln!(
        "[vmm] paging active — identity 0–4MiB, higher-half kernel mapped"
    );

    // ── VMM smoke-test: map a scratch page, write to it, unmap it ─────────
    {
        let test_virt = vmm::VirtAddr(0xFFFF_A000_0001_0000); // higher-half scratch VA
        let test_phys = pmm::alloc_frame().expect("vmm smoke-test: no frame");
        vmm::map_page(test_virt, test_phys, vmm::PageFlags::KERNEL_RW);
        // Write a sentinel through the mapping and read it back.
        unsafe {
            let p = test_virt.as_u64() as *mut u64;
            p.write_volatile(0xDEAD_BEEF_CAFE_BABE);
            assert_eq!(
                p.read_volatile(),
                0xDEAD_BEEF_CAFE_BABE,
                "vmm smoke-test: sentinel mismatch"
            );
        }
        vmm::unmap_page(test_virt);
        pmm::free_frame(test_phys);
    }
    kprintln!("[vmm] smoke-test passed");

    // ── Heap allocator ────────────────────────────────────────────────────
    heap::init();
    kprintln!(
        "[heap] initialized — {} KiB pre-mapped at {:#x}",
        heap::HEAP_INIT_PAGES * 4,
        heap::HEAP_START,
    );

    // ── Heap smoke-test ───────────────────────────────────────────────────
    {
        // Box<T>: single heap allocation, dealloc on drop.
        let b = Box::new(0xDEAD_BEEF_u64);
        assert_eq!(*b, 0xDEAD_BEEF_u64, "heap smoke-test: Box value mismatch");
        drop(b);

        // Vec<T>: heap-backed growable array.
        let mut v = Vec::<u8>::with_capacity(256);
        for i in 0..64_u8 {
            v.push(i);
        }
        assert_eq!(v.len(), 64, "heap smoke-test: Vec length mismatch");
        assert_eq!(v[0], 0,  "heap smoke-test: Vec[0] mismatch");
        assert_eq!(v[63], 63, "heap smoke-test: Vec[63] mismatch");
    }
    kprintln!("[heap] smoke-test passed");

    // ── Scheduler ─────────────────────────────────────────────────────────
    task::init();
    kprintln!("[sched] initialized");

    task::spawn_kernel_task(task_b);

    // Cooperative yield smoke-test: task_a (this thread) and task_b alternate.
    // Expected interleaving:
    //   task A tick 0 → task B tick 0 → task A tick 1 → task B tick 1
    //   → task A tick 2 → task B tick 2 → task A: smoke-test passed
    for i in 0..3_u32 {
        kprintln!("[task A] tick {}, yielding...", i);
        task::yield_task();
    }
    kprintln!("[sched] smoke-test passed");

    // ── APIC + preemptive timer ───────────────────────────────────────────
    apic::init();
    kprintln!("[apic] timer active — preemptive scheduling enabled");

    // Smoke-test: sleep ~50 ms by polling the tick counter.
    let t0 = apic::ticks();
    while apic::ticks() < t0 + 50 {
        unsafe { core::arch::asm!("hlt") };
    }
    kprintln!("[apic] smoke-test passed — {} ticks elapsed", apic::ticks() - t0);

    // ── Syscall interface ─────────────────────────────────────────────────
    syscall::init();
    kprintln!("[syscall] initialized — SMEP on, LSTAR/STAR/FMASK configured");

    // ── Capability system ─────────────────────────────────────────────────
    {
        let mut alice = cap::CapabilityTable::new();
        let mut bob   = cap::CapabilityTable::new();

        // Create a physical memory object and give alice a root cap (all rights).
        let obj = cap::create_object(cap::KernelObject::Memory {
            base_pa: 0x1000, frame_count: 1,
        }).expect("cap smoke-test: create_object");

        let h_alice = cap::create_root_cap(
            &mut alice, cap::CapKind::Memory, cap::CapRights::ALL, obj,
        );

        // Alice grants Bob a read-only derived capability.
        let h_bob = cap::cap_grant(
            &mut alice, h_alice,
            99, // placeholder task id for bob
            &mut bob,
            cap::CapRights::READ,
        ).expect("cap smoke-test: cap_grant");

        // Bob's cap carries only READ — WRITE/GRANT/REVOKE were masked off.
        assert_eq!(
            bob.get(h_bob).expect("cap smoke-test: get bob cap").rights,
            cap::CapRights::READ,
            "cap smoke-test: rights mismatch",
        );

        // Bob cannot re-grant (no Grant right).
        assert!(
            cap::cap_grant(&mut bob, h_bob, 0, &mut alice, cap::CapRights::READ).is_err(),
            "cap smoke-test: Bob should not be able to grant",
        );

        // Alice revokes her root cap.
        cap::cap_revoke(&mut alice, h_alice).expect("cap smoke-test: revoke");

        // Alice's handle is now invalid.
        assert!(
            alice.get(h_alice).is_err(),
            "cap smoke-test: cap should be gone after revoke",
        );
        // Bob's derived cap is still present (cascading revocation is separate).
        assert!(
            bob.get(h_bob).is_ok(),
            "cap smoke-test: Bob's derived cap should survive a single-table revoke",
        );
    }
    kprintln!("[cap] smoke-test passed");

    // ── Cascade-revoke smoke-test ─────────────────────────────────────────
    {
        let mut alice = cap::CapabilityTable::new();
        let mut bob   = cap::CapabilityTable::new();

        let obj = cap::create_object(cap::KernelObject::Memory {
            base_pa: 0x3000, frame_count: 1,
        }).expect("cascade smoke: create_object");

        let h_alice = cap::create_root_cap(
            &mut alice, cap::CapKind::Memory, cap::CapRights::ALL, obj,
        );
        let h_bob = cap::cap_grant(
            &mut alice, h_alice, 99, &mut bob, cap::CapRights::READ,
        ).expect("cascade smoke: cap_grant");

        // Alice cascade-revokes her root cap → bob's derived cap disappears too.
        let bob_ptr: *mut cap::CapabilityTable = &mut bob;
        cap::cap_cascade_revoke(&mut alice, h_alice, &mut |tid| {
            if tid == 99 { bob_ptr } else { core::ptr::null_mut() }
        }).expect("cascade smoke: revoke");

        assert!(alice.get(h_alice).is_err(), "cascade: alice's cap should be gone");
        assert!(bob.get(h_bob).is_err(),     "cascade: bob's derived cap should be gone");
    }
    kprintln!("[cap] cascade-revoke smoke-test passed");

    // ── Userspace entry smoke-test ────────────────────────────────────────
    // Spawn a kernel task that maps a user code page, writes `mov eax,1;
    // syscall` into it (SYS_TASK_EXIT = 1), and enters ring 3.  The syscall
    // handler calls task_exit(), marks the task Dead, and switches back to
    // kmain.
    task::spawn_kernel_task(userspace_smoke_task);
    task::yield_task();
    kprintln!("[syscall] userspace entry smoke-test passed");

    kprintln!("Boot complete.");

    loop { unsafe { core::arch::asm!("hlt") }; }
}

/// Kernel task for the Step 10 userspace smoke-test.
///
/// Maps a user code page and a user stack page, writes two instructions
/// (`mov eax, SYS_TASK_EXIT; syscall`) into the code page, then enters
/// ring 3.  The syscall handler calls `task_exit()`, which marks this task
/// Dead and switches back to kmain.
fn userspace_smoke_task() -> ! {
    // Allocate and map a user-executable code page.
    let code_phys = pmm::alloc_frame().expect("userspace smoke: no frame for code");
    let code_va   = vmm::VirtAddr(0x0000_0001_0000_0000);
    vmm::map_page(code_va, code_phys, vmm::PageFlags::USER_RX);

    // Write: `mov eax, 1` (SYS_TASK_EXIT); `syscall`
    unsafe {
        let p = code_va.as_u64() as *mut u8;
        p.add(0).write(0xB8);                           // MOV EAX, imm32
        p.add(1).write(syscall::SYS_TASK_EXIT as u8);   // imm32 byte 0
        p.add(2).write(0x00);
        p.add(3).write(0x00);
        p.add(4).write(0x00);
        p.add(5).write(0x0F);                           // SYSCALL (two-byte opcode)
        p.add(6).write(0x05);
    }

    // Allocate and map a user stack page.
    let stack_phys = pmm::alloc_frame().expect("userspace smoke: no frame for stack");
    let stack_va   = vmm::VirtAddr(0x0000_0002_0000_0000);
    vmm::map_page(stack_va, stack_phys, vmm::PageFlags::USER_RW);
    let stack_top  = vmm::VirtAddr(stack_va.as_u64() + 4096);

    // Enter ring 3 — never returns (user code calls SYS_TASK_EXIT).
    syscall::enter_userspace(code_va, stack_top);
}

/// Second kernel task: prints three ticks interleaved with task A, then exits.
fn task_b() -> ! {
    for i in 0..3_u32 {
        kprintln!("[task B] tick {}, yielding...", i);
        task::yield_task();
    }
    kprintln!("[task B] done, exiting");
    task::task_exit();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("[PANIC] {}", info);
    loop { unsafe { core::arch::asm!("hlt") }; }
}
