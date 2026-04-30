---
source_file: "src/task.rs"
type: "code"
community: "Kernel Core Subsystems"
location: "L1"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Core_Subsystems
---

# task.rs

## Connections
- [[GlobalSched]] - `contains` [EXTRACTED]
- [[Scheduler]] - `contains` [EXTRACTED]
- [[Task]] - `contains` [EXTRACTED]
- [[TaskContext]] - `contains` [EXTRACTED]
- [[TaskState]] - `contains` [EXTRACTED]
- [[block_and_yield()]] - `contains` [EXTRACTED]
- [[block_task()]] - `contains` [EXTRACTED]
- [[cap_table_ptr()]] - `contains` [EXTRACTED]
- [[check_stack_canary()]] - `contains` [EXTRACTED]
- [[current_entry_and_stack()]] - `contains` [EXTRACTED]
- [[current_kernel_stack_top()]] - `contains` [EXTRACTED]
- [[current_page_table()]] - `contains` [EXTRACTED]
- [[current_task_id()]] - `contains` [EXTRACTED]
- [[for_each_task()]] - `contains` [EXTRACTED]
- [[get_sched()]] - `contains` [EXTRACTED]
- [[init()_5]] - `contains` [EXTRACTED]
- [[kill_task()]] - `contains` [EXTRACTED]
- [[set_bootstrap_cap_table()]] - `contains` [EXTRACTED]
- [[spawn_kernel_task()]] - `contains` [EXTRACTED]
- [[spawn_userspace_task()]] - `contains` [EXTRACTED]
- [[sweep_dead()]] - `contains` [EXTRACTED]
- [[switch_cr3()]] - `contains` [EXTRACTED]
- [[task_exists()]] - `contains` [EXTRACTED]
- [[task_exit()]] - `contains` [EXTRACTED]
- [[task_status_raw()]] - `contains` [EXTRACTED]
- [[vma_insert()]] - `contains` [EXTRACTED]
- [[vma_remove()]] - `contains` [EXTRACTED]
- [[wake_task()]] - `contains` [EXTRACTED]
- [[yield_task()]] - `contains` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Core_Subsystems