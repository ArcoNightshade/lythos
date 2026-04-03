# lythos / RaptorOS — next steps

The kernel (lythos) and init process (lythd + lythdist) boot cleanly to
`[integration] all checks passed`. This list tracks what comes next.

---

## 1. Wrap SYS_IPC_SEND_CAP / SYS_IPC_RECV_CAP in lythos-std

**What:** The kernel implements SYS_IPC_SEND_CAP (12) and SYS_IPC_RECV_CAP (13)
for transferring a capability handle alongside a 64-byte message. lythos-std
exposes neither — every service handshake that delivers a cap must encode the
handle as raw bytes and re-derive it.

**Fix:** Add `sys_ipc_send_cap(ep_cap, msg, cap_to_send)` and
`sys_ipc_recv_cap(ep_cap, buf) -> (usize, Option<u64>)` to
`lythos-std/src/lib.rs`, then expose them on `ipc::Endpoint` as
`send_with_cap` / `recv_with_cap`.

**Files:** `RaptorOS/lythos-std/src/lib.rs`, `RaptorOS/lythos-std/src/ipc.rs`

---

## 2. lythdist: register itself with the service registry

**What:** lythdist is spawned by lythd and starts its loop, but it never
registers itself with lythd's service registry. Any future service that needs
a memory capability has no standard way to discover lythdist.

**Fix:** Before entering its main loop, lythdist should send a `KIND_REGISTER`
message to the registry endpoint. The registry cap needs to be passed in as a
fourth capability (handle 3) at spawn time.

**Changes:**
- `lythd/src/main.rs` — pass `registry.as_raw()` as `caps[3]` when spawning
  lythdist.
- `lythdist/src/main.rs` — `const REGISTRY_CAP: u64 = 3;` at entry; send a
  `Register` frame for the name `"lythdist"` before the main loop.

**Files:** `RaptorOS/lythd/src/main.rs`, `RaptorOS/lythdist/src/main.rs`

---

## 3. Wrap SYS_SERIAL_READ in lythos-std

**What:** The kernel exposes SYS_SERIAL_READ (14) — a blocking read from
COM1 that returns when at least one byte is available. lythos-std has no
wrapper, so no userspace program can read keyboard or serial input.

**Fix:** Add `sys_serial_read(buf: &mut [u8]) -> Result<usize, SysError>` to
`lythos-std/src/lib.rs`. Wire it into the existing `io::Read` trait so an
`Endpoint::read()` call works with `BufReader`.

**Files:** `RaptorOS/lythos-std/src/lib.rs`, `RaptorOS/lythos-std/src/io.rs`

---

## 4. Add SYS_CLOCK (APIC tick counter) to kernel + lythos-std

**What:** `lythos-std/src/time.rs` has a comment: *"Instant is a stub — lythos
has no wall-clock syscall yet."* The kernel already maintains an APIC tick
counter (`apic::tick_count()`); it just isn't exposed via syscall.

**Fix (kernel):** Add `SYS_CLOCK = 15` to `src/syscall.rs`. The handler reads
`apic::tick_count()` and returns it as a raw `u64`. Tick period is
`1_000_000 / APIC_TIMER_HZ` nanoseconds (the calibrated value from
`apic::init`).

**Fix (lythos-std):** Implement `Instant` using `SYS_CLOCK`:
`Instant::now()` calls `sys_clock()`, `elapsed()` returns a `Duration`.

**Files:** `src/syscall.rs`, `src/apic.rs`, `RaptorOS/lythos-std/src/lib.rs`,
`RaptorOS/lythos-std/src/time.rs`

---

## 5. Implement lythmsg — inter-service message router

**What:** lythd's boot comment mentions "lythdist, lythmsg" as the two core
services. lythmsg doesn't exist yet. It should be a brokered publish/subscribe
or request/reply router: services register named channels, clients send typed
requests, lythmsg forwards them and delivers replies. This separates endpoint
discovery from IPC.

**Design sketch:**
- Protocol: 64-byte frames, first byte = kind
  (0=Subscribe, 1=Publish, 2=Request, 3=Reply, 4=Ack, 5=Nack)
- lythmsg holds a name→endpoint map (like lythd's service table but for
  message topics).
- Services subscribe a local IPC endpoint to a named topic.
- Senders publish to a topic; lythmsg fans out to all subscribers.

**Fix:**
- Create `RaptorOS/lythmsg/` crate (copy skeleton from lythdist).
- Add to workspace `Cargo.toml`.
- Add `static LYTHMSG_ELF` to `lythos/src/elf.rs` (via `include_bytes!`).
- Spawn lythmsg from `lythd/src/main.rs` after lythdist.
- Register `"lythmsg"` in the service registry.

**Files:** `RaptorOS/lythmsg/` (new), `RaptorOS/Cargo.toml`,
`lythos/src/elf.rs`, `RaptorOS/lythd/src/main.rs`

---

## 6. First real userspace application — lysh (lythos shell)

**What:** All infrastructure is in place for a minimal interactive program:
serial read (item 3), exec (SYS_EXEC), service registry (lythd), and
capabilities. A simple REPL that reads a line from COM1, parses it as a
command name + args, looks up the command binary via the service registry,
and spawns it would exercise the entire stack end-to-end.

**Scope (v0):**
- Built-in commands only: `help`, `echo <text>`, `exit`.
- Uses `sys_serial_read` + a line buffer.
- Prints output via `println!`.
- No filesystem (binaries embedded as statics, looked up by name).

**Files:** `RaptorOS/lysh/` (new), `RaptorOS/Cargo.toml`,
`RaptorOS/lythd/src/main.rs` (spawn lysh after lythdist)

---

## 7. Per-process memory isolation smoke test

**What:** `exec` creates per-process PML4s, but there's no test verifying that
one process cannot read another's pages. A malicious or buggy process writing
to another's VA range should get a `#PF`, not silent success.

**Fix:** Add a kernel-level smoke test in `src/elf.rs` or `src/main.rs`:
1. Spawn process A at the standard VA (`0x1_0000_0000`).
2. From a kernel thread, attempt to read A's VA without mapping it in the
   kernel page table — should fault.
3. Verify that spawning a second process B at the same VA maps *different*
   physical frames (query both page tables).

Log result as `[isolation] per-process page table check passed`.

**Files:** `src/main.rs` (or `src/elf.rs`), possibly `src/vmm.rs`
(`query_page_in` is already available)
