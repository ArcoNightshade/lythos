/// Cooperative task scheduler — kernel tasks and context switching.
///
/// ## Context switch mechanics
///
/// `switch_context(from, to)` is a small assembly routine (AT&T syntax).
/// Calling convention: SysV AMD64 — `rdi = from`, `rsi = to`.
///
/// On entry the `call` has already pushed the return address.  The routine
/// pushes `{rbp, rbx, r12, r13, r14, r15}`, saves `rsp → from->rsp`, loads
/// `to->rsp`, pops the six registers in reverse order, and `ret`s — which
/// pops the next task's `rip` from the top of its stack.
///
/// ## Initial stack frame for new tasks
///
/// A new task's stack is pre-populated so the first restore looks identical
/// to every subsequent one.  After `switch_context` runs 6 pops (48 bytes)
/// and `ret` (8 bytes):
///
///   `rsp_entry = initial_rsp + 56`
///
/// SysV ABI requires `rsp_entry % 16 == 8`
/// → `initial_rsp % 16 == 0`  (since 56 % 16 == 8).
///
/// `KERNEL_STACK_SIZE` is a multiple of 16, and our heap allocator guarantees
/// 16-byte alignment, so `stack_top` is always 16-byte aligned.
/// `initial_rsp = stack_top − 64` (64 % 16 == 0) satisfies the constraint.
///
/// ```text
/// [initial_rsp +  0]  r15 = 0           ← context.rsp points here
/// [initial_rsp +  8]  r14 = 0
/// [initial_rsp + 16]  r13 = 0
/// [initial_rsp + 24]  r12 = 0
/// [initial_rsp + 32]  rbx = 0
/// [initial_rsp + 40]  rbp = 0
/// [initial_rsp + 48]  entry address     ← popped by `ret` → rip
/// [initial_rsp + 56]  (8-byte padding)
/// ```

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::arch::global_asm;
use core::cell::UnsafeCell;

// ── Assembly: context switch ──────────────────────────────────────────────────

global_asm!(r#"
.global switch_context
.type   switch_context, @function
switch_context:
    pushq  %rbp
    pushq  %rbx
    pushq  %r12
    pushq  %r13
    pushq  %r14
    pushq  %r15
    movq   %rsp, (%rdi)
    movq   (%rsi), %rsp
    popq   %r15
    popq   %r14
    popq   %r13
    popq   %r12
    popq   %rbx
    popq   %rbp
    retq
"#, options(att_syntax));

unsafe extern "C" {
    fn switch_context(from: *mut TaskContext, to: *const TaskContext);
}

// ── Types ─────────────────────────────────────────────────────────────────────

pub type TaskId = u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskState {
    Running,
    Ready,
}

/// Per-task saved scheduler state.  Only `rsp` is stored; all other
/// callee-saved registers live on the task's kernel stack between switches.
#[repr(C)]
pub struct TaskContext {
    pub rsp: u64,
}

pub const KERNEL_STACK_SIZE: usize = 16 * 1024; // 16 KiB per task

pub struct Task {
    pub id:      TaskId,
    pub state:   TaskState,
    pub context: TaskContext,
    /// Heap allocation backing this task's kernel stack.  Must not be resized
    /// after spawn; `context.rsp` points into this buffer.
    _stack: Vec<u8>,
}

// ── Scheduler ─────────────────────────────────────────────────────────────────

struct Scheduler {
    /// All tasks.  Stored as `Box<Task>` so each Task's heap address is stable
    /// even if the Vec reallocates.
    tasks:   Vec<Box<Task>>,
    current: usize,
    next_id: TaskId,
}

// Use UnsafeCell + a newtype to avoid `static mut` (which Rust 2024 makes
// harder to use safely with the `static_mut_refs` lint).
struct GlobalSched(UnsafeCell<Option<Scheduler>>);
// SAFETY: single-threaded kernel; no concurrent accesses.
unsafe impl Sync for GlobalSched {}

static SCHED: GlobalSched = GlobalSched(UnsafeCell::new(None));

/// Get a `&mut Scheduler`, panicking if `init()` has not been called.
#[inline]
unsafe fn get_sched() -> &'static mut Scheduler {
    unsafe { (*SCHED.0.get()).as_mut().expect("task: scheduler not initialised") }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialise the scheduler.  Creates a bootstrap task for the running
/// `kmain` thread.  Must be called before `spawn_kernel_task` / `yield_task`.
pub fn init() {
    let bootstrap = Box::new(Task {
        id:      0,
        state:   TaskState::Running,
        context: TaskContext { rsp: 0 }, // filled by the first switch_context
        _stack:  Vec::new(),             // kmain uses the existing boot stack
    });

    let mut tasks = Vec::new();
    tasks.push(bootstrap);

    unsafe {
        *SCHED.0.get() = Some(Scheduler { tasks, current: 0, next_id: 1 });
    }
}

/// Spawn a new kernel-mode task beginning execution at `entry`.
///
/// `entry` must never return (infinite loop or eventual `task_exit` in Step 10).
pub fn spawn_kernel_task(entry: fn() -> !) -> TaskId {
    // Allocate a zeroed kernel stack.  The heap allocator guarantees 16-byte
    // alignment, so stack_top = base + KERNEL_STACK_SIZE is also aligned.
    let mut stack = Vec::with_capacity(KERNEL_STACK_SIZE);
    stack.resize(KERNEL_STACK_SIZE, 0u8);

    let stack_top   = stack.as_ptr() as usize + KERNEL_STACK_SIZE;
    let initial_rsp = stack_top - 64; // 64 % 16 == 0 → initial_rsp is 16-byte aligned

    // Layout: 8 × u64 slots from initial_rsp upward (see module doc).
    unsafe {
        let p = initial_rsp as *mut u64;
        p.add(0).write(0);              // r15
        p.add(1).write(0);              // r14
        p.add(2).write(0);              // r13
        p.add(3).write(0);              // r12
        p.add(4).write(0);              // rbx
        p.add(5).write(0);              // rbp
        p.add(6).write(entry as u64);   // rip — popped by `ret`
        p.add(7).write(0);              // padding
    }

    let sched = unsafe { get_sched() };
    let id    = sched.next_id;
    sched.next_id += 1;

    sched.tasks.push(Box::new(Task {
        id,
        state:   TaskState::Ready,
        context: TaskContext { rsp: initial_rsp as u64 },
        _stack:  stack,
    }));

    id
}

/// Cooperative yield: save the current task and switch to the next ready task
/// in round-robin order.  Returns when this task is switched back to.
/// No-op if there are no other ready tasks.
pub fn yield_task() {
    let sched = unsafe { get_sched() };

    let n       = sched.tasks.len();
    let current = sched.current;

    if n <= 1 { return; }

    // Find the next task in Ready state (linear scan, wrapping).
    let mut next = (current + 1) % n;
    loop {
        if sched.tasks[next].state == TaskState::Ready { break; }
        next = (next + 1) % n;
        if next == current { return; }
    }

    // Capture raw pointers to both TaskContexts *before* mutating state.
    // Box<Task> is heap-allocated; the Task address is stable across Vec ops.
    let from_ctx: *mut   TaskContext = &mut sched.tasks[current].context;
    let to_ctx:   *const TaskContext = &    sched.tasks[next].context;

    sched.tasks[current].state = TaskState::Ready;
    sched.tasks[next].state    = TaskState::Running;
    sched.current              = next;

    // After this call returns we are back in `current`'s context.
    unsafe { switch_context(from_ctx, to_ctx); }
}
