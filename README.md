# lythos

A capability-based microkernel written in Rust, targeting bare-metal x86_64.
Built from scratch.

---

## What is this?

Lythos is a kernel built around three ideas:

1. **Capability tokens** — every resource (memory region, IPC endpoint, device) is accessed through an unforgeable kernel-managed handle. No ambient authority.
2. **A stable service model** — a PID-1 supervisor (`lythd`) owns process lifecycle, dependency ordering, and a 30-second automatic rollback window on bad updates.
3. **Explicit, auditable boot** — no magic; the boot sequence is a straight line of initialised subsystems, each smoke-tested before the next begins.

The kernel is being built in fourteen discrete steps. Steps 1–7 are complete.

---

## Architecture

```
                        ┌─────────────────────────────────────┐
                        │             Userspace               │
                        │  lythd  lythdist  lythmsg  services │
                        └──────────────┬──────────────────────┘
                                       │  syscall / sysret
                        ┌──────────────▼──────────────────────┐
                        │          Kernel (ring 0)            │
                        │                                     │
                        │  ┌────────┐  ┌──────┐  ┌────────┐   │
                        │  │  IPC   │  │ ELF  │  │ lythd  │   │
                        │  │ Step11 │  │Step13│  │Step12  │   │
                        │  └────────┘  └──────┘  └────────┘   │
                        │  ┌────────┐  ┌────────────────────┐ │
                        │  │  Caps  │  │    Userspace I/F   │ │
                        │  │ Step 9 │  │      Step 10       │ │
                        │  └────────┘  └────────────────────┘ │
                        │  ┌────────┐  ┌────────┐             │
                        │  │  APIC  │  │ Sched  │ ← Step 7    │
                        │  │ Step 8 │  │  RR    │             │
                        │  └────────┘  └────────┘             │
                        │  ┌────────┐  ┌────────┐             │
                        │  │  Heap  │  │  VMM   │ ← Step 6    │
                        │  │ linked │  │4-level │ ← Step 5    │
                        │  └────────┘  └────────┘             │
                        │  ┌────────┐  ┌────────┐             │
                        │  │  PMM   │  │ Serial │ ← Step 4    │
                        │  │ bitmap │  │ kprint │ ← Step 3    │
                        │  └────────┘  └────────┘             │
                        │  ┌────────┐  ┌────────┐             │
                        │  │  GDT   │  │  IDT   │ ← Step 2    │
                        │  └────────┘  └────────┘             │
                        │  ┌──────────────────────────────┐   │
                        │  │  Boot stub (Multiboot2, ASM) │ ← Step 1
                        │  └──────────────────────────────┘   │
                        └─────────────────────────────────────┘
```

---

## Progress

| Step | Subsystem                                                               | Status |
| :--: | ----------------------------------------------------------------------- | :----: |
|  1   | Bare-metal boot target (Multiboot2, 32→64-bit stub, custom target spec) |   X    |
|  2   | GDT, IDT, CPU exception handling                                        |   X    |
|  3   | Serial output, spinlock, `kprint!` / `kprintln!`                        |   X    |
|  4   | Physical memory manager (bitmap allocator, Multiboot1+2 parser)         |   X    |
|  5   | Virtual memory — 4-level paging, `map_page` / `unmap_page`, NXE         |   X    |
|  6   | Kernel heap (`GlobalAlloc` linked-list, `Box` / `Vec` support)          |   X    |
|  7   | Cooperative scheduler, kernel tasks, context switching                  |   X    |
|  8   | APIC + preemption timer                                                 |        |
|  9   | Capability system (`CapHandle`, `CapabilityTable`, grant/revoke)        |        |
|  10  | Userspace (ring 3, `syscall`/`sysret`, SMEP/SMAP)                       |        |
|  11  | IPC — shared-memory ring buffers, `lythmsg` substrate                   |        |
|  12  | `lythd` (PID 1, service supervisor, 30-second rollback timer)           |        |
|  13  | ELF loader, `exec`, static userspace binaries                           |        |
|  14  | Full boot integration and smoke-test suite                              |        |

---

## Subsystems (implemented)

### Boot — `src/arch/x86_64/boot.s`

AT&T assembly stub that accepts both Multiboot1 (QEMU `-kernel`) and Multiboot2 (GRUB2) loaders. Transitions from 32-bit protected mode to 64-bit long mode: enables PAE, builds a temporary identity-mapped P4/P3/P2 table using 2 MiB huge pages, sets `EFER.LME`, enables paging, loads a 64-bit GDT, and calls `kmain`.

### GDT & IDT — `src/gdt.rs`, `src/idt.rs`

Three-descriptor GDT (null, kernel code, kernel data). 256-entry IDT wired to per-vector stubs in `isr_stubs.s`; each stub pushes a canonical error code and vector number before jumping to the shared `exception_handler`. Ring 3 descriptors are added in Step 10.

### Exceptions — `src/exceptions.rs`

`exception_handler` receives a pointer to `ExceptionFrame` (all GPRs + vector + error code + CPU-pushed RIP/CS/RFLAGS). Page faults (vector 14) read `CR2` and decode the `P/W/U` error bits before halting. All other exceptions log diagnostics and halt.

### Serial & Logging — `src/serial.rs`

16550A UART driver for COM1 (I/O port `0x3F8`), 115200 8N1, no interrupts. Implements `core::fmt::Write`. Wrapped in a TTAS spinlock (`AtomicBool`) so `kprintln!` is safe to call from any context without heap involvement.

### Physical Memory Manager — `src/pmm.rs`

Bitmap allocator: one bit per 4 KiB frame, covering 4 GiB (128 KiB of bitmap). Parses the Multiboot1 or Multiboot2 memory map, marks the kernel image and BIOS area as used, then exposes `alloc_frame() → Option<PhysAddr>` and `free_frame(PhysAddr)`. A `PhysAddr` newtype prevents mixing physical and virtual addresses.

### Virtual Memory Manager — `src/vmm.rs`

Four-level paging. The identity map (0–1 GiB) is built with 2 MiB huge pages, requiring only three frames — PML4, PDPT, PD — which means every future PMM allocation is immediately accessible without a chicken-and-egg problem. The kernel image is additionally mapped into the higher-half window at `0xFFFF_8000_0000_0000`. `EFER.NXE` is enabled before any NX-flagged entry is written. `map_page` / `unmap_page` work at 4 KiB granularity on all non-identity-mapped addresses and invalidate the TLB via `invlpg`.

### Heap — `src/heap.rs`

1024 × 4 KiB pages (4 MiB) pre-mapped at `0xFFFF_C000_0000_0000` on first use. A first-fit linked-list allocator implements `GlobalAlloc`; blocks carry a 16-byte header (`size` + `next`). Blocks are split when the remainder is large enough; dealloc reinserts at the list head in O(1). All allocations are 16-byte aligned. Registered as `#[global_allocator]`, making `alloc::boxed::Box`, `alloc::vec::Vec`, `alloc::sync::Arc`, etc. available throughout the kernel.

### Scheduler — `src/task.rs`

Cooperative round-robin over a `Vec<Box<Task>>`. Each task owns a 16 KiB heap-allocated kernel stack. Only `rsp` is stored in `TaskContext` — the other callee-saved registers (`rbp`, `rbx`, `r12`–`r15`) live on the task's stack between switches.

`switch_context` is twelve lines of inline AT&T assembly: push six registers, save `rsp` into `from→rsp`, load `rsp` from `to→rsp`, pop six registers, `ret`. The initial stack frame for a new task is laid out so the first restore is indistinguishable from every subsequent one. Stack alignment is verified: `initial_rsp % 16 == 0`, giving `rsp_entry % 16 == 8` at the entry function — correct per SysV AMD64.

`yield_task()` finds the next `Ready` task, swaps states, and calls `switch_context`. Preemption (APIC timer → `schedule()`) is wired in Step 8.

---

## Boot output

```
lythos kernel initializing...
[gdt] loaded
[idt] loaded - exceptions active
[pmm] initialized — 32556 free frames (127 MiB)
[pmm] smoke-test passed
[vmm] paging active — identity 0–4MiB, higher-half kernel mapped
[vmm] smoke-test passed
[heap] initialized — 4096 KiB pre-mapped at 0xffffc00000000000
[heap] smoke-test passed
[sched] initialized
[task A] tick 0, yielding...
[task B] tick 0, yielding...
[task A] tick 1, yielding...
[task B] tick 1, yielding...
[task A] tick 2, yielding...
[task B] tick 2, yielding...
[sched] smoke-test passed
Boot complete.
```

---

## Building

**Requirements**

- Rust nightly toolchain (automatically selected via `rust-toolchain.toml`)
- `rust-src` and `llvm-tools` components (pulled in by `rust-toolchain.toml`)
- QEMU (`qemu-system-x86_64`) for testing

**Build**

```bash
cargo build
```

The `.cargo/config.toml` supplies the target (`x86_64-lythos.json`), the linker script, and the `build-std` flags automatically.

**Run under QEMU**

```bash
qemu-system-x86_64 \
  -kernel target/x86_64-lythos/debug/lythos \
  -serial stdio \
  -display none
```

Add `-d int,cpu_reset` to log all interrupts and CPU resets — useful when debugging triple faults.

---

## Repository layout

```
lythos/
├── src/
│   ├── main.rs              # kmain, boot orchestration, smoke tests
│   ├── serial.rs            # 16550A UART driver, spinlock, kprint! macro
│   ├── gdt.rs               # Global Descriptor Table
│   ├── idt.rs               # Interrupt Descriptor Table
│   ├── exceptions.rs        # Exception frame, #PF handler
│   ├── pmm.rs               # Physical memory manager (bitmap)
│   ├── vmm.rs               # Virtual memory manager (4-level paging)
│   ├── heap.rs              # Kernel heap (linked-list GlobalAlloc)
│   ├── task.rs              # Tasks, context switch, cooperative scheduler
│   └── arch/
│       └── x86_64/
│           ├── boot.s       # Multiboot2 header, 32→64-bit stub
│           └── isr_stubs.s  # Exception stubs (vectors 0–31)
├── x86_64-lythos.json       # Custom Rust target specification
├── linker.ld                # Kernel linker script (load at 0x100000)
├── rust-toolchain.toml      # Pins nightly + required components
├── Cargo.toml               # Crate manifest (no_std, panic = abort)
└── .cargo/
    └── config.toml          # Target, linker flags, build-std config
```

---

## Design notes

**Why a capability system?**
Capabilities make the security model explicit and auditable. Instead of checking whether a caller _is allowed_ to access a resource (ACL style), the kernel checks whether the caller _holds a token_ for it. Tokens cannot be forged, can be delegated with narrowed rights, and can be revoked. This makes isolation between services straightforward without a complex policy engine.

**Why a 30-second rollback timer?**
Atomic updates are only useful if bad updates can be undone automatically. `lythd` watches critical services during a stability window after each update. If a critical service crashes before the window closes, it initiates a Btrfs subvolume rollback and reboots. If all services stay up, it clears the rollback flag. No manual intervention required.

**Why 2 MiB huge pages for the identity map?**
A conventional 4 KiB identity map covering 128 MiB needs 32 768 page-table frames — all of which must already be accessible to set up. 2 MiB huge pages (PS=1 at the PD level) require only three frames: PML4, PDPT, PD. This breaks the chicken-and-egg dependency entirely and leaves the allocator free to hand out frames for other purposes.

**Why a cooperative scheduler first?**
Cooperative scheduling is much easier to reason about: no ISR stack-switching, no race conditions, no guard around `yield_task`. It is also sufficient to validate the context-switch machinery. Preemption is added in Step 8 by making the APIC timer ISR call `schedule()` — at which point the cooperative path becomes a strict subset.

---

## License

MIT
