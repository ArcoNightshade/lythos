---
type: community
cohesion: 0.13
members: 22
---

# IDT & Interrupt Descriptors

**Cohesion:** 0.13 - loosely connected
**Members:** 22 nodes

## Members
- [[.absent()]] - code - src/idt.rs
- [[.init()]] - code - src/serial.rs
- [[.interrupt()]] - code - src/idt.rs
- [[.lock()]] - code - src/serial.rs
- [[.new()_4]] - code - src/serial.rs
- [[.try_read_byte()]] - code - src/serial.rs
- [[.write_byte()]] - code - src/serial.rs
- [[.write_str()]] - code - src/serial.rs
- [[IdtEntry]] - code - src/idt.rs
- [[IdtPtr]] - code - src/idt.rs
- [[SerialPort]] - code - src/serial.rs
- [[SpinLock]] - code - src/serial.rs
- [[SpinLockT]] - code - src/serial.rs
- [[SpinLockGuard]] - code - src/serial.rs
- [[idt.rs]] - code - src/idt.rs
- [[inb()_2]] - code - src/serial.rs
- [[init()_2]] - code - src/idt.rs
- [[init()_8]] - code - src/serial.rs
- [[outb()_2]] - code - src/serial.rs
- [[register_irq()]] - code - src/idt.rs
- [[remap_and_mask_pic()]] - code - src/idt.rs
- [[serial.rs]] - code - src/serial.rs

## Live Query (requires Dataview plugin)

```dataview
TABLE source_file, type FROM #community/IDT_&_Interrupt_Descriptors
SORT file.name ASC
```

## Connections to other communities
- 1 edge to [[_COMMUNITY_APIC Hardware Interface]]
- 1 edge to [[_COMMUNITY_Kernel Core Subsystems]]

## Top bridge nodes
- [[register_irq()]] - degree 3, connects to 1 community
- [[SpinLockT]] - degree 2, connects to 1 community