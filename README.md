# lythos

**A capability-based microkernel for x86_64, written entirely in Rust.**

Lythos is a bare-metal kernel built on the principle that security should be structural — not bolted on. Every resource is gated behind an unforgeable capability token. Every update carries an automatic rollback window. Every boot step is smoke-tested before the next begins.

---

## Why lythos?

Most kernels enforce security through permissions checked at the time of access. Lythos enforces security through *possession*: if you don't hold the token, you cannot even express the request.

| Traditional kernel | lythos |
|--------------------|--------|
| "Is this process allowed to access X?" | "Does this process hold a capability for X?" |
| Ambient authority — processes inherit broad rights | Least-privilege by default — rights are explicitly granted and can be narrowed |
| Policy lives in a complex ACL/permission engine | Policy is the capability table itself — auditable, unforgeable |
| Bad updates require manual rollback | 30-second automatic rollback window on every update |

---

## Core design

### Capability tokens

Every resource in lythos — a memory region, an IPC endpoint, a rollback ticket — is accessed through a kernel-managed `CapHandle`. Handles are opaque to userspace and cannot be forged. They can be delegated to other tasks with narrowed rights, and they can be revoked, cascading to all derived holders. This gives you a clean, auditable delegation graph with no ambient authority anywhere in the system.

```
                      CapKind::Memory    CapKind::Ipc      CapKind::Rollback
                           │                  │                   │
                     rights: ALL        rights: READ|WRITE   rights: ALL
                           │
                    cap_grant(to=lythdist, rights: READ)
                           │
                     rights: READ   ← derived handle, cannot escalate
```

### `lythd` — a supervisor that cleans up after itself

`lythd` is PID 1. It owns process lifecycle and dependency ordering for the entire userspace. After each update it opens a **30-second stability window**. If a critical service crashes before the window closes, `lythd` initiates a Btrfs subvolume rollback and reboots automatically. No operator intervention. No partial-update wedged state.

### Explicit, auditable boot

Lythos boots in fourteen discrete, smoke-tested steps. Each subsystem must pass its own test before the next is initialized. If anything fails, the kernel halts with a diagnostic — it never silently continues in a broken state.

---

## Architecture

```
                  ┌─────────────────────────────────────────────┐
                  │                  Userspace                  │
                  │      lythd   lythdist   lythmsg   services  │
                  └──────────────────┬──────────────────────────┘
                                     │  syscall / sysret
                  ┌──────────────────▼──────────────────────────┐
                  │              Kernel  (ring 0)               │
                  │                                             │
                  │  ┌──────────────────────────────────────┐  │
                  │  │  Integration & smoke-test suite       │  │  Step 14
                  │  └──────────────────────────────────────┘  │
                  │  ┌───────────┐  ┌────────────────────────┐ │
                  │  │ ELF loader│  │  lythd boot protocol   │ │  Steps 12–13
                  │  └───────────┘  └────────────────────────┘ │
                  │  ┌───────────┐  ┌────────────────────────┐ │
                  │  │    IPC    │  │   Userspace interface  │ │  Steps 10–11
                  │  │ring buffer│  │  syscall/sysret, SMEP  │ │
                  │  └───────────┘  └────────────────────────┘ │
                  │  ┌───────────┐  ┌────────────────────────┐ │
                  │  │   Caps    │  │    APIC + preemption   │ │  Steps 8–9
                  │  │ grant/rev │  │    calibrated timer    │ │
                  │  └───────────┘  └────────────────────────┘ │
                  │  ┌───────────┐  ┌────────────────────────┐ │
                  │  │   Heap    │  │    Scheduler (RR)      │ │  Steps 6–7
                  │  └───────────┘  └────────────────────────┘ │
                  │  ┌───────────┐  ┌────────────────────────┐ │
                  │  │    VMM    │  │   PMM (bitmap alloc)   │ │  Steps 4–5
                  │  │ 4-level   │  │                        │ │
                  │  └───────────┘  └────────────────────────┘ │
                  │  ┌───────────┐  ┌────────────────────────┐ │
                  │  │   GDT     │  │   IDT / exceptions     │ │  Steps 2–3
                  │  └───────────┘  └────────────────────────┘ │
                  │  ┌──────────────────────────────────────┐  │
                  │  │   Boot stub  (Multiboot1/2, ASM)     │  │  Step 1
                  │  └──────────────────────────────────────┘  │
                  └─────────────────────────────────────────────┘
```

---

## What's inside

### Memory management

- **PMM** — bitmap allocator, one bit per 4 KiB frame, covering 4 GiB. Parses both Multiboot1 and Multiboot2 memory maps.
- **VMM** — 4-level paging with NX support. The identity map (0–1 GiB) uses 2 MiB huge pages, requiring only three frames at boot — no chicken-and-egg problem with the allocator. Kernel image additionally mapped into the higher-half window at `0xFFFF_8000_0000_0000`.
- **Heap** — linked-list `GlobalAlloc` backed by the VMM. Enables `Box`, `Vec`, `Arc` throughout the kernel without a separate allocator crate.

### Scheduling

Round-robin scheduler over kernel tasks. Each task owns a 16 KiB stack. `switch_context` is twelve lines of inline assembly: push six registers, swap stack pointers, pop six registers, `ret`. Preemption is driven by the Local APIC timer.

### IPC

Each IPC endpoint is a 4 KiB shared physical page. A fixed 64-byte message slot ring buffer sits inside (`RING_CAPACITY = 63`). `send` blocks if the ring is full; `recv` blocks if it is empty. The kernel maps each endpoint at a deterministic address in the IPC window — no dynamic lookup at message time.

### Syscall interface

| Nr | Name | Purpose |
|----|------|---------|
| 0 | `SYS_YIELD` | Cooperative yield |
| 1 | `SYS_TASK_EXIT` | Terminate the calling task |
| 2 | `SYS_MMAP` | Map a physical frame into the calling task |
| 3 | `SYS_MUNMAP` | Unmap a virtual address |
| 4 | `SYS_CAP_GRANT` | Delegate a capability with narrowed rights |
| 5 | `SYS_CAP_REVOKE` | Revoke a capability |
| 6 | `SYS_IPC_SEND` | Send a message to an endpoint |
| 7 | `SYS_IPC_RECV` | Receive a message from an endpoint |
| 8 | `SYS_IPC_CREATE` | Create a new IPC endpoint |
| 9 | `SYS_ROLLBACK` | Trigger a system rollback (requires Rollback cap) |
| 10 | `SYS_EXEC` | Load and spawn an ELF64 binary |

Entry via `syscall`/`sysret`. SMEP and SMAP active. Interrupts disabled during syscall dispatch.

---

## Boot output

Lythos boots to a clean integration pass on every build:

```
lythos kernel initializing...
[gdt] loaded
[idt] loaded - exceptions active
[pmm] initialized — 32556 free frames (127 MiB)
[pmm] smoke-test passed
[vmm] paging active — identity 0–1GiB, higher-half kernel mapped
[vmm] smoke-test passed
[heap] initialized — 4096 KiB pre-mapped at 0xffffc00000000000
[heap] smoke-test passed
[sched] initialized
[apic] timer calibrated — 1000 Hz
[apic] smoke-test passed
[caps] initialized
[caps] smoke-test passed
[syscall] LSTAR/STAR/FMASK configured
[userspace] smoke-test passed
[ipc] initialized — ring capacity 63 × 64 B
[ipc] smoke-test passed
[lythd] loaded — boot cap queued, rollback timer armed
[elf] loader smoke-test passed
[integration] all checks passed
```

---

## Build & run

**Requirements**

- Rust nightly (auto-selected via `rust-toolchain.toml`)
- `rust-src` and `llvm-tools` components (auto-installed)
- `qemu-system-x86_64`

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run
qemu-system-x86_64 -kernel target/x86_64-lythos/debug/lythos -serial stdio -display none

# Debug triple faults
qemu-system-x86_64 -kernel target/x86_64-lythos/debug/lythos -serial stdio -display none -d int,cpu_reset
```

---

## Repository layout

```
lythos/
├── src/
│   ├── main.rs              # kmain, boot sequence, integration smoke-test
│   ├── serial.rs            # 16550A UART, spinlock, kprint! / kprintln!
│   ├── gdt.rs               # Global Descriptor Table
│   ├── idt.rs               # Interrupt Descriptor Table, PIC remapping
│   ├── exceptions.rs        # Exception handler, page-fault diagnostics
│   ├── pmm.rs               # Physical memory manager (bitmap)
│   ├── vmm.rs               # Virtual memory manager (4-level paging)
│   ├── heap.rs              # Kernel heap (linked-list GlobalAlloc)
│   ├── task.rs              # Tasks, context switch, round-robin scheduler
│   ├── apic.rs              # Local APIC init, calibrated preemption timer
│   ├── syscall.rs           # syscall_entry (asm), dispatch, enter_userspace
│   ├── cap.rs               # Capability table, grant, revoke, cascade revoke
│   ├── ipc.rs               # IPC endpoints, ring buffers, blocking send/recv
│   ├── elf.rs               # ELF64 loader, exec(), stack allocation
│   └── arch/x86_64/
│       ├── boot.s           # Multiboot1/2, 32→64-bit stub, BSS zero, page tables
│       └── isr_stubs.s      # ISR stubs for vectors 0–31
├── x86_64-lythos.json       # Custom Rust target spec
├── linker.ld                # Kernel memory layout (load at 0x100000)
├── rust-toolchain.toml      # Nightly pin + required components
└── Cargo.toml               # no_std, panic = abort
```

---

## License

MIT
