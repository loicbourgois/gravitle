#!/usr/bin/env python3

import argparse
import fcntl
import hashlib
import os
from pathlib import Path
import signal
import subprocess
import sys
import re
import time
from collections import deque
from datetime import datetime, timezone
import traceback


CLI_DIR = Path(__file__).resolve().parent
STATE_DIR = (
    Path(os.environ.get("XDG_STATE_HOME") or Path.home() / ".local/state")
    / "gravitle-cli"
)
LOCK_PATH = STATE_DIR / "watch.lock"
CONTROL_PATH = STATE_DIR / "watch-control.lock"
LOG_PATH = CLI_DIR / "logs" / "watch.log"
LOG_LIMIT = 100


def format_log_paths(message):
    home = re.escape(str(Path.home()))
    return re.sub(
        home + r"""(?=/|$|[\s)'":,;\]}])""",
        "~",
        str(message),
    )


def log_message(message):
    LOG_PATH.parent.mkdir(parents=True, exist_ok=True)

    try:
        with LOG_PATH.open("r", encoding="utf-8", errors="replace") as stream:
            lines = deque(
                (format_log_paths(line) for line in stream),
                maxlen=LOG_LIMIT,
            )
    except FileNotFoundError:
        lines = deque(maxlen=LOG_LIMIT)

    for line in format_log_paths(message).splitlines():
        timestamp = datetime.now(timezone.utc).isoformat(
            timespec="seconds"
        ).replace("+00:00", "Z")
        lines.append(f"{timestamp} {line}\n")

    with LOG_PATH.open("w", encoding="utf-8") as stream:
        stream.writelines(lines)


def try_lock(stream):
    try:
        fcntl.flock(stream.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        return False
    return True


def snapshot():
    entries = {}

    for root, directories, files in os.walk(CLI_DIR):
        directories[:] = [
            name
            for name in directories
            if name not in {".git", "__pycache__"}
            and not (
                Path(root) == CLI_DIR
                and name in {"target", "logs"}
            )
        ]

        for name in files:
            path = Path(root) / name
            try:
                metadata = path.stat()
                digest = hashlib.sha256(path.read_bytes()).digest()
            except FileNotFoundError:
                # Editors can replace or delete files during a scan.
                continue

            entries[str(path.relative_to(CLI_DIR))] = (
                metadata.st_mode,
                metadata.st_mtime_ns,
                digest,
            )

    return entries


def install():
    log_message("Building and installing the Gravitle CLI...")

    try:
        with subprocess.Popen(
            ["zsh", str(CLI_DIR / "install.sh")],
            cwd=CLI_DIR,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            encoding="utf-8",
            errors="replace",
        ) as process:
            for line in process.stdout:
                log_message(line)

            returncode = process.wait()
    except OSError as error:
        log_message(f"Could not start installation: {error}")
        return

    if returncode:
        log_message("Build/install failed; waiting for another change.")
    else:
        log_message("Installed g and completions. Watching for changes...")


def watch(lock_fd):
    # Only this process should retain the lifetime lock, not build helpers.
    os.set_inheritable(lock_fd, False)

    try:
        previous = snapshot()
        install()

        while True:
            time.sleep(0.5)
            current = snapshot()

            if current != previous:
                # Preserve edits made during installation for the next poll.
                previous = current
                install()
    except Exception:
        log_message(traceback.format_exc())
        raise
    finally:
        os.close(lock_fd)


def start(lock):
    if not try_lock(lock):
        print("Gravitle CLI watch is already active.")
        return 0

    # Prepare the destination before reporting a successful start.
    LOG_PATH.parent.mkdir(parents=True, exist_ok=True)
    LOG_PATH.touch(exist_ok=True)

    process = subprocess.Popen(
        [
            sys.executable,
            "-u",
            str(Path(__file__).resolve()),
            "--worker-fd",
            str(lock.fileno()),
        ],
        cwd=CLI_DIR,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        start_new_session=True,
        pass_fds=(lock.fileno(),),
    )

    # Management commands hold the control lock until this is published.
    lock.seek(0)
    lock.truncate()
    lock.write(str(process.pid))
    lock.flush()

    print(f"Started Gravitle CLI watch (PID {process.pid}).")
    print(f"Log: {LOG_PATH}")
    return 0


def status(lock):
    if try_lock(lock):
        print("Gravitle CLI watch is inactive.")
        return 1

    lock.seek(0)
    print(f"Gravitle CLI watch is active (PID {lock.read().strip()}).")
    print(f"Log: {LOG_PATH}")
    return 0


def stop(lock):
    if try_lock(lock):
        print("Gravitle CLI watch is already inactive.")
        return 0

    lock.seek(0)
    pid = int(lock.read().strip())

    try:
        # The worker starts a new session; include any running build helpers.
        os.killpg(pid, signal.SIGTERM)
    except ProcessLookupError:
        pass

    deadline = time.monotonic() + 10
    while not try_lock(lock):
        if time.monotonic() >= deadline:
            print(
                "Watcher did not stop within 10 seconds.",
                file=sys.stderr,
            )
            return 1
        time.sleep(0.1)

    print("Stopped Gravitle CLI watch.")
    return 0


def main():
    parser = argparse.ArgumentParser(
        description="Manage background rebuilding of the Gravitle CLI."
    )
    parser.add_argument(
        "action",
        nargs="?",
        choices=("start", "status", "stop"),
        default="start",
    )
    parser.add_argument("--worker-fd", type=int, help=argparse.SUPPRESS)
    args = parser.parse_args()

    if args.worker_fd is not None:
        watch(args.worker_fd)
        return 0

    STATE_DIR.mkdir(parents=True, exist_ok=True)

    # Serialize management commands, including publication of the worker PID.
    with CONTROL_PATH.open("a+") as control:
        fcntl.flock(control.fileno(), fcntl.LOCK_EX)

        with LOCK_PATH.open("a+") as lock:
            return {
                "start": start,
                "status": status,
                "stop": stop,
            }[args.action](lock)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError) as error:
        print(f"Gravitle CLI watch: {error}", file=sys.stderr)
        sys.exit(1)