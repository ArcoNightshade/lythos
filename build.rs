// Build script: compile mkrfs and create disk.img for QEMU.
//
// The disk image is written to disk.img in the project root.
// Pass it to QEMU with:
//   -drive file=disk.img,format=raw,if=none,id=hd0
//   -device virtio-blk-pci,drive=hd0

fn main() {
    // Build mkrfs using its own Makefile (rustc directly — avoids build-std config clash).
    let status = std::process::Command::new("make")
        .args(["-C", "tools/mkrfs"])
        .status()
        .expect("failed to invoke make for mkrfs");
    assert!(status.success(), "mkrfs build failed");

    // Create disk.img from rootfs/ if it exists, or an empty 64 MiB image.
    let mut cmd = std::process::Command::new("tools/mkrfs/mkrfs");
    cmd.args(["disk.img", "64M"]);
    if std::path::Path::new("rootfs").is_dir() {
        cmd.arg("rootfs");
    }
    let status = cmd.status().expect("failed to run mkrfs");
    assert!(status.success(), "disk image creation failed");

    println!("cargo:rerun-if-changed=tools/mkrfs/src/main.rs");
    watch_dir("rootfs");
}

fn watch_dir(dir: &str) {
    println!("cargo:rerun-if-changed={}", dir);
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());
            if path.is_dir() {
                watch_dir(&path.to_string_lossy());
            }
        }
    }
}
