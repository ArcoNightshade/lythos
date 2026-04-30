---
type: community
cohesion: 0.07
members: 76
---

# Kernel Core Subsystems

**Cohesion:** 0.07 - loosely connected
**Members:** 76 nodes

## Members
- [[.as_u64()]] - code - src/pmm.rs
- [[.insert()]] - code - src/cap.rs
- [[.new()_2]] - code - src/cap.rs
- [[.new()_5]] - code - src/serial.rs
- [[.ring_ptr()]] - code - src/ipc.rs
- [[BootInfo]] - code - src/main.rs
- [[EpTable]] - code - src/ipc.rs
- [[ExceptionFrame]] - code - src/exceptions.rs
- [[GlobalSched]] - code - src/task.rs
- [[IpcEndpoint]] - code - src/ipc.rs
- [[PhysAddr]] - code - src/pmm.rs
- [[RingBuffer]] - code - src/ipc.rs
- [[Scheduler]] - code - src/task.rs
- [[Task]] - code - src/task.rs
- [[TaskContext]] - code - src/task.rs
- [[TaskState]] - code - src/task.rs
- [[alloc_cap_id()]] - code - src/cap.rs
- [[alloc_frame()]] - code - src/pmm.rs
- [[block_and_yield()]] - code - src/task.rs
- [[block_task()]] - code - src/task.rs
- [[build_boot_info()]] - code - src/main.rs
- [[cap_grant()]] - code - src/cap.rs
- [[cap_inherit()]] - code - src/cap.rs
- [[cap_table_ptr()]] - code - src/task.rs
- [[check_stack_canary()]] - code - src/task.rs
- [[core_smoke()]] - code - src/main.rs
- [[cpuid_vendor()]] - code - src/main.rs
- [[create_endpoint()]] - code - src/ipc.rs
- [[create_object()]] - code - src/cap.rs
- [[create_root_cap()]] - code - src/cap.rs
- [[current_entry_and_stack()]] - code - src/task.rs
- [[current_kernel_stack_top()]] - code - src/task.rs
- [[current_page_table()]] - code - src/task.rs
- [[current_task_id()]] - code - src/task.rs
- [[endpoint_phys()]] - code - src/ipc.rs
- [[enter_userspace()]] - code - src/syscall.rs
- [[ep_table()]] - code - src/ipc.rs
- [[exception_handler()]] - code - src/exceptions.rs
- [[exceptions.rs]] - code - src/exceptions.rs
- [[exec_trampoline()]] - code - src/elf.rs
- [[for_each_task()]] - code - src/task.rs
- [[free_frame_count()]] - code - src/pmm.rs
- [[get_sched()]] - code - src/task.rs
- [[init()_1]] - code - src/heap.rs
- [[init()_5]] - code - src/task.rs
- [[ipc.rs]] - code - src/ipc.rs
- [[kernel_pml4()]] - code - src/vmm.rs
- [[kill_task()]] - code - src/task.rs
- [[kmain()]] - code - src/main.rs
- [[main.rs_1]] - code - src/main.rs
- [[map_page()]] - code - src/vmm.rs
- [[page_fault_handler()]] - code - src/exceptions.rs
- [[panic()]] - code - src/main.rs
- [[recv()]] - code - src/ipc.rs
- [[recv_cap()]] - code - src/ipc.rs
- [[send()]] - code - src/ipc.rs
- [[send_cap()]] - code - src/ipc.rs
- [[set_bootstrap_cap_table()]] - code - src/task.rs
- [[set_rsp0()]] - code - src/tss.rs
- [[spawn_kernel_task()]] - code - src/task.rs
- [[spawn_userspace_task()]] - code - src/task.rs
- [[sweep_dead()]] - code - src/task.rs
- [[switch_cr3()]] - code - src/task.rs
- [[syscall_dispatch()]] - code - src/syscall.rs
- [[task.rs]] - code - src/task.rs
- [[task_b()]] - code - src/main.rs
- [[task_exists()]] - code - src/task.rs
- [[task_exit()]] - code - src/task.rs
- [[task_status_raw()]] - code - src/task.rs
- [[ticks()]] - code - src/apic.rs
- [[unmap_page()]] - code - src/vmm.rs
- [[userspace_smoke_task()]] - code - src/main.rs
- [[vma_insert()]] - code - src/task.rs
- [[vma_remove()]] - code - src/task.rs
- [[wake_task()]] - code - src/task.rs
- [[yield_task()]] - code - src/task.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/Kernel_Core_Subsystems
SORT file.name ASC
```

## Connections to other communities
- 38 edges to [[_COMMUNITY_ELF Loader & Process Launch]]
- 22 edges to [[_COMMUNITY_Capability System (Code)]]
- 12 edges to [[_COMMUNITY_Physical Memory Manager]]
- 6 edges to [[_COMMUNITY_APIC Hardware Interface]]
- 4 edges to [[_COMMUNITY_Syscall Entry & Dispatch]]
- 2 edges to [[_COMMUNITY_Kernel Heap Allocator]]
- 2 edges to [[_COMMUNITY_IOAPIC Driver]]
- 1 edge to [[_COMMUNITY_GDT & Segment Setup]]
- 1 edge to [[_COMMUNITY_SpinLock Guard]]
- 1 edge to [[_COMMUNITY_IDT & Interrupt Descriptors]]

## Top bridge nodes
- [[syscall_dispatch()]] - degree 44, connects to 5 communities
- [[kmain()]] - degree 27, connects to 4 communities
- [[PhysAddr]] - degree 19, connects to 4 communities
- [[map_page()]] - degree 13, connects to 3 communities
- [[alloc_frame()]] - degree 11, connects to 2 communities