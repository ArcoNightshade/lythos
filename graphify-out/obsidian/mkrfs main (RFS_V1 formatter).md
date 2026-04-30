---
source_file: "tools/mkrfs/src/main.rs"
type: "code"
community: "Kernel Initialization"
location: "line 747"
tags:
  - graphify/code
  - graphify/EXTRACTED
  - community/Kernel_Initialization
---

# mkrfs main (RFS_V1 formatter)

## Connections
- [[Allocator (block bitmap allocator)]] - `calls` [EXTRACTED]
- [[Disk (image file IO)]] - `calls` [EXTRACTED]
- [[InodeTable (RFS inode table)]] - `calls` [EXTRACTED]
- [[populate (recursive dir builder)]] - `calls` [EXTRACTED]
- [[write_superblock]] - `calls` [EXTRACTED]

#graphify/code #graphify/EXTRACTED #community/Kernel_Initialization