# CASK (Capability-Aware System Kernel) — outstanding work

## Hardening

- [x] Audit syscall surface for integer overflow and TOCTOU issues:
  - **SYS_MMAP / SYS_MUNMAP missing VA range check** — VA in 0→1 GiB would panic (`walk_or_create` hits PS=1 huge-page entry); kernel-space VA would corrupt shared intermediate page-table entries (U/S propagation). Fixed: reject VA < 1 GiB or ≥ `0x0000_8000_0000_0000`.
  - **No TOCTOU** — IF=0 during syscall handling + single CPU means no check→use races exist on the current design.
- [x] Rate-limit IPC endpoint creation — added `MAX_ENDPOINTS = 1024` cap in `ipc::create_endpoint()`; returns `None` when reached; `SYS_IPC_CREATE` returns EINVAL. Also eliminates the latent `kern_virt` overflow at `idx > MAX_ENDPOINTS`.
- [ ] KASLR — randomise kernel physical load address at boot
- [x] Guard against stack overflow in kernel tasks — PMM-backed guard pages already present; added software canary (`STACK_CANARY = 0x5AFE_C0DE_DEAD_BEEF`) written at stack bottom on spawn. Checked at `yield_task`, `block_and_yield`, `task_exit`, and `sweep_dead`. Catches shallow overflows that don't reach the guard page.

## Features

- [ ] `SYS_MMAP` variant that maps a caller-named physical address (needed for device drivers in userspace)
- [ ] SMP — per-CPU scheduler queues, per-CPU APIC, spinlocks replacing interrupt-mask sections
- [ ] x2APIC support (required on hardware with > 255 logical CPUs)
- [ ] ACPI table parsing — replace hardcoded QEMU assumptions with real hardware discovery

## Testing

- [x] Release build validation — passes cleanly; fixed two bugs found in the process:
  - BSS zeroing used floor division (`shr $2`) so the last 1–3 bytes were skipped if BSS size wasn't a multiple of 4. Fixed by adding `add $3` before the shift (ceiling division). `APIC_ONLINE` was the affected symbol.
  - PMM double-free in `sweep_dead` due to APIC timer re-entrancy. Fixed by masking interrupts (pushfq/cli/.../popfq) for the duration of the sweep.
- [x] Additional integration tests (added to `core_smoke`):
  - cap grant / revoke: SYS_CAP_GRANT self-grant → EINVAL, grant to nonexistent task → EINVAL, SYS_CAP_REVOKE → 0
  - `SYS_EXEC` invoked from a userspace task — `EXEC_FROM_USER_ELF` calls SYS_EXEC with an embedded SMOKE_ELF
  - `SYS_MMAP` / `SYS_MUNMAP` full lifecycle — `MMAP_TEST_ELF` maps, writes, unmaps; frame freed on exit
  - `SYS_IPC_SEND_CAP` / `SYS_IPC_RECV_CAP` end-to-end — kernel task transfers a Memory cap through the ring buffer
  - Blocked task woken by a third task (triangular IPC) — tasks A and B block; task C wakes both
- [x] Fuzz syscall entry with boundary/invalid inputs — added to `core_smoke`:
  - Unknown syscall numbers (14, 15, 100, 255, u64::MAX) → ENOSYS
  - SYS_MMAP/SYS_MUNMAP unaligned VA → EINVAL; SYS_MUNMAP unmapped VA → EINVAL
  - Null buf_ptr for all buffer syscalls (IPC_SEND/RECV/SEND_CAP/RECV_CAP) → EINVAL
  - SYS_IPC_RECV_CAP kernel-space out_handle_ptr → EINVAL
  - SYS_EXEC null/overflow elf_ptr → EINVAL; SYS_LOG null/kernel/oversized ptr → EINVAL
  - Bogus cap handles (SYS_CAP_GRANT, SYS_CAP_REVOKE) → ENOCAP
- [x] QEMU `-d int,cpu_reset` clean run on release build — passes, no spurious faults; fixed one bug in the process:
  - `SYSCALL_KERN_RSP` (shared global) was overwritten by each task's `enter_userspace`, causing the wrong kernel stack to be used when multiple concurrent userspace tasks made syscalls. Fixed by updating `SYSCALL_KERN_RSP` and `tss::RSP0` on every context switch via `switch_cr3` in `task.rs`.

## Cleanup

- [x] `userspace_smoke_task` frame leak fixed — frames saved to `SMOKE_CODE_PHYS`/`SMOKE_STACK_PHYS` statics; kmain unmaps and frees them after the task exits
- [x] `NEXT_STACK_SLOT` exhaustion — added `assert!(slot < 511)` in `alloc_user_stack_into`; available VA window is 0x7FFF_0000_0000→0x8000_0000_0000 (4 GiB), slot size 8 MiB, limit 511 slots
