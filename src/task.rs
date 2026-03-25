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
    /// Waiting on an event (e.g. IPC). Will not be scheduled until wake_task is called.
    Blocked,
    /// Exited. Will be freed on the next yield_task sweep.
    Dead,
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
/// `entry` must never return; call `task_exit()` when done.
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

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Remove all Dead tasks from the queue, adjusting `current` accordingly.
/// Must be called when the current task is Running (never Dead).
fn sweep_dead(sched: &mut Scheduler) {
    let mut i = 0;
    while i < sched.tasks.len() {
        if sched.tasks[i].state == TaskState::Dead {
            sched.tasks.remove(i);
            if i < sched.current {
                sched.current -= 1;
            }
            // don't advance i — the element at i is now the next task
        } else {
            i += 1;
        }
    }
}

// ── Public API (continued) ────────────────────────────────────────────────────

/// Cooperative yield: save the current task and switch to the next ready task
/// in round-robin order.  Returns when this task is switched back to.
/// No-op if there are no other ready tasks.  Frees any Dead tasks first.
pub fn yield_task() {
    let sched = unsafe { get_sched() };
    sweep_dead(sched);

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

/// Terminate the current task.  Marks it Dead, switches to the next ready task,
/// and never returns.  If no other task is ready, halts the CPU.
pub fn task_exit() -> ! {
    let sched = unsafe { get_sched() };
    let current = sched.current;

    sched.tasks[current].state = TaskState::Dead;

    // Find the next Ready task.
    let n = sched.tasks.len();
    let mut next = (current + 1) % n;
    loop {
        if sched.tasks[next].state == TaskState::Ready { break; }
        next = (next + 1) % n;
        if next == current {
            // No other ready tasks — halt.
            loop { unsafe { core::arch::asm!("hlt") }; }
        }
    }

    // from_ctx is written by switch_context but never read again — the Dead
    // task's stack stays valid until sweep_dead drops it on the next yield.
    let from_ctx: *mut   TaskContext = &mut sched.tasks[current].context;
    let to_ctx:   *const TaskContext = &    sched.tasks[next].context;

    sched.tasks[next].state = TaskState::Running;
    sched.current           = next;

    unsafe { switch_context(from_ctx, to_ctx); }

    unreachable!("task_exit: returned from switch_context")
}

/// Block the task with `id`, removing it from scheduling until `wake_task` is called.
/// No-op if the task is not in the Ready state.
pub fn block_task(id: TaskId) {
    let sched = unsafe { get_sched() };
    if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == id) {
        if t.state == TaskState::Ready {
            t.state = TaskState::Blocked;
        }
    }
}

/// Wake a blocked task, making it eligible to be scheduled again.
/// No-op if the task is not in the Blocked state.
pub fn wake_task(id: TaskId) {
    let sched = unsafe { get_sched() };
    if let Some(t) = sched.tasks.iter_mut().find(|t| t.id == id) {
        if t.state == TaskState::Blocked {
            t.state = TaskState::Ready;
        }
    }
}
