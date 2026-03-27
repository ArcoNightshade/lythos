# lythos / RaptorOS — build TODO

All kernel hardening items are complete. This list tracks what is needed
to compile and boot a real lythd userspace process.

---

## 1. RaptorOS `.cargo/config.toml`

**What:** The RaptorOS workspace has no `.cargo/config.toml`, so `cargo build`
uses the host target and ignores `userspace.ld`.

**Fix:** Create `.cargo/config.toml` that sets the default build target to
`x86_64-raptoros` and passes `-T userspace.ld` to the linker for all
executables.

**Files:** `RaptorOS/.cargo/config.toml`

---

## 2. `lythos-std` Cargo.toml + crate root

**What:** `lythos-std/Cargo.toml` has no `[lib]` section and `src/` is empty.
`lythd` can't compile without it.

**Fix:** Update `Cargo.toml` (add `build-std` dependency chain markers), create
`src/lib.rs` that wires together all sub-modules and sets `#![no_std]`.

**Files:** `RaptorOS/lythos-std/Cargo.toml`, `RaptorOS/lythos-std/src/lib.rs`

---

## 3. Raw syscall layer

**What:** Every `lythos-std` API ultimately calls a lythos syscall via the
`syscall` instruction. Nothing works without this foundation.

**Fix:** `src/syscall.rs` — one `unsafe` inline-asm function per syscall
number (SYS_YIELD through SYS_IPC_RECV_CAP = 0–13), each returning a raw
`u64`. Include the error-sentinel constants and a `SysError(u64)` newtype
with `is_err()` and `into_result()`.

**Files:** `RaptorOS/lythos-std/src/syscall.rs`

---

## 4. `BootInfo`

**What:** `lythd` calls `BootInfo::from_bytes(&frame)` on the 64-byte message
received from the kernel boot endpoint.

**Fix:** `src/boot.rs` — 64-byte layout struct, `from_bytes()` with signature
check (`0xB007_1000_B007_1000`), `vendor_str()` (12-byte CPUID string).

**Files:** `RaptorOS/lythos-std/src/boot.rs`

---

## 5. `ipc::Endpoint`

**What:** `lythd` calls `Endpoint::create()`, `Endpoint::from_raw()`,
`endpoint.recv_frame()`, and `endpoint.send()`.

**Fix:** `src/ipc.rs` — newtype wrapping a `u64` cap handle; `create()` calls
SYS_IPC_CREATE; `send()` calls SYS_IPC_SEND; `recv_frame()` calls
SYS_IPC_RECV and returns `Result<[u8; 64], SysError>`.

**Files:** `RaptorOS/lythos-std/src/ipc.rs`

---

## 6. I/O: `println!` / `eprintln!` / `sys_log`

**What:** `lythd` uses `println!` and `eprintln!` for all output, and
`sys_log` directly in the panic handler. None of these exist yet.

**Fix:** `src/io.rs` — `sys_log(s: &str)` calls SYS_LOG; `print!` /
`println!` / `eprintln!` format into a stack buffer then call `sys_log`.
Implement `core::fmt::Write` on a zero-size writer that flushes via SYS_LOG.

**Files:** `RaptorOS/lythos-std/src/io.rs`

---

## 7. Task utilities

**What:** `lythd` calls `task::yield_now()`, `sys_task_exit()`, and
`sys_rollback()`. The panic handler calls `sys_task_exit()` after rollback.

**Fix:** `src/task.rs` — thin wrappers: `yield_now()` → SYS_YIELD,
`sys_task_exit() -> !` → SYS_TASK_EXIT, `sys_rollback() -> !` → SYS_ROLLBACK,
`spawn(elf: &[u8], caps: &[u64]) -> Result<u64, SysError>` → SYS_EXEC.

**Files:** `RaptorOS/lythos-std/src/task.rs`

---

## 8. Global allocator

**What:** `lythd` uses `Vec<Service>` which requires a `#[global_allocator]`.
Without one, the crate won't link.

**Fix:** `src/allocator.rs` — bump allocator that calls SYS_MMAP to claim
4 KiB pages on demand. `dealloc` is a no-op (lythd's service table only
grows). Register with `#[global_allocator]`.

**Files:** `RaptorOS/lythos-std/src/allocator.rs`

---

## 9. Build lythd + kernel integration test

**What:** Once items 1–8 are done, lythd should compile to a static ELF64.
The kernel's `src/elf.rs` already has:

```rust
pub static LYTHD_ELF: &[u8] =
    include_bytes!("../../RaptorOS/target/x86_64-raptoros/release/lythd");
```

**Fix:** Build RaptorOS (`cargo build --release` in `RaptorOS/`), then build
lythos (`cargo build` in `lythos/`), boot under QEMU and verify output:

```
[lythd] lythos init — X MiB free (N frames), cpu: GenuineIntel
[lythd] service registry online (cap 3)
[lythd] core services pending (lythdist, lythmsg not yet built)
[lythd] entering supervisor loop
```

**Files:** `RaptorOS/` (build), `lythos/` (rebuild + boot)
