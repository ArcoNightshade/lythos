#!/usr/bin/env bash
set -e

OROS="${OROS:-$HOME/Documents/GitHub/RaptorOS}"
LYTHOS="$(cd "$(dirname "$0")" && pwd)"
RELEASE="${RELEASE:-}"

# ── build flags ──────────────────────────────────────────────────────────────
if [[ -n "$RELEASE" ]]; then
    CARGO_FLAGS="--release"
    KERNEL_BIN="$LYTHOS/target/x86_64-lythos/release/lythos"
else
    CARGO_FLAGS=""
    KERNEL_BIN="$LYTHOS/target/x86_64-lythos/debug/lythos"
fi

# ── build userspace ───────────────────────────────────────────────────────────
echo "[run.sh] building OROS userspace..."
(cd "$OROS" && cargo build -p lythdist --release -q)
(cd "$OROS" && cargo build -p lysh     --release -q)
(cd "$OROS" && cargo build -p lythd    --release -q)

# ── build kernel ──────────────────────────────────────────────────────────────
echo "[run.sh] building lythos kernel..."
(cd "$LYTHOS" && cargo build $CARGO_FLAGS -q)

# ── run ───────────────────────────────────────────────────────────────────────
# Use a Unix-domain socket for the serial port so QEMU delivers every
# keystroke reliably (stdio/nographic can silently drop RX on macOS).
# nc(1) is built into macOS and handles the socket connection.
SOCK="/tmp/lythos-serial-$$.sock"

cleanup() {
    kill "$QPID" 2>/dev/null || true
    rm -f "$SOCK"
}
trap cleanup EXIT

echo "[run.sh] launching QEMU..."
qemu-system-x86_64 \
    -kernel "$KERNEL_BIN" \
    -chardev socket,id=s0,path="$SOCK",server=on,wait=on \
    -serial chardev:s0 \
    -display none \
    "$@" &
QPID=$!

# Wait for QEMU to create the listening socket before connecting Python.
for i in $(seq 1 40); do
    [ -S "$SOCK" ] && break
    sleep 0.1
done

if [ ! -S "$SOCK" ]; then
    echo "[run.sh] error: QEMU socket not created" >&2
    exit 1
fi

# Connect in raw mode so every keystroke goes straight through.
# isig kept on so Ctrl+C still works as a kill signal.
echo "[run.sh] connected — Ctrl+C to quit"

# Write the bridge to a temp file so Python's stdin is not consumed by a heredoc.
BRIDGE="$(mktemp /tmp/lythos-bridge-XXXX)"
cat > "$BRIDGE" <<'PYEOF'
import socket, sys, os, select, termios

path = sys.argv[1]
sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.connect(path)

# Ensure stdin is raw (in case the shell's stty didn't propagate).
old_attrs = termios.tcgetattr(0)
new_attrs = termios.tcgetattr(0)
new_attrs[0] &= ~(termios.IGNBRK | termios.BRKINT | termios.PARMRK |
                  termios.ISTRIP | termios.INLCR | termios.IGNCR |
                  termios.ICRNL | termios.IXON)
new_attrs[1] |=  termios.OPOST | termios.ONLCR
new_attrs[2] &= ~termios.CSIZE
new_attrs[2] |=  termios.CS8
new_attrs[3] &= ~(termios.ECHO | termios.ECHONL | termios.ICANON | termios.IEXTEN)
new_attrs[3] |=  termios.ISIG
new_attrs[6][termios.VMIN]  = 1
new_attrs[6][termios.VTIME] = 0
termios.tcsetattr(0, termios.TCSADRAIN, new_attrs)

try:
    while True:
        r, _, _ = select.select([0, sock], [], [])
        if 0 in r:
            data = os.read(0, 256)
            if not data:
                break
            sock.sendall(data)
        if sock in r:
            data = sock.recv(4096)
            if not data:
                break
            os.write(1, data)
except (KeyboardInterrupt, BrokenPipeError, OSError):
    pass
finally:
    termios.tcsetattr(0, termios.TCSADRAIN, old_attrs)
PYEOF

python3 "$BRIDGE" "$SOCK"
rm -f "$BRIDGE"
