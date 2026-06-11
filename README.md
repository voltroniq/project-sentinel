# Project Sentinel | Official Companion Repository

**Master Systems Programming in Rust.**

Project Sentinel is a 17-chapter Rust systems programming curriculum that
progresses from basic CLI parsing to advanced Linux kernel observability.
This repository is the official companion for the
[Project Sentinel](https://www.voltroniq.com/project-sentinel) course.

### How to use this repository
- **Volume 0 (Preludes 1–4)** — Fully open source.
- **Volume 1 (Chapters 1–4)** — Fully open source.
- **Volumes 2 & 3** — Reference code for enrolled students.

*For the full step-by-step guidance visit
[voltroniq.com/project-sentinel](https://voltroniq.com/project-sentinel)*

---

### Folder Structure

#### `/volume-0-prelude`
Single file with commented examples for all four preludes.
- `src/main.rs`
- **Status:** 100% Complete — MIT Licensed.

#### `/volume-1-foundations`
Fully functional CLI threat scanner.
- `src/main.rs`
- **Status:** 100% Complete — MIT Licensed.

#### `/volume-2-systems`
Modular, concurrent threat ledger with file persistence.
- `src/main.rs`
- `src/models.rs`
- `src/engine.rs`
- **Status:** Reference implementation for enrolled students.

#### `/volume-3-kernel`
Linux Namespace isolation and eBPF kernel supervisor.
- `sentinel-core/src/main.rs`
- `sentinel-core/src/sandbox.rs`
- `sentinel-ebpf/src/main.rs`
- **Status:** Reference implementation for enrolled students.
- *[Enroll in the course](https://www.voltroniq.com/project-sentinel)
  to unlock the full pedagogical breakdown.*

---

### System Prerequisites

1. **Rust Toolchain:** `rustup` latest stable.
2. **Linux Environment:** Required for Volume 3 (Namespaces & eBPF).
   WSL2 on Windows works if your kernel is 5.15+.
3. **Root privileges:** Volume 3 must run as `sudo`.

---

### Running the Code

```bash
# Volume 0
cd volume-0-prelude && cargo run

# Volume 1
cd volume-1-foundations && cargo run

# Volume 2
cd volume-2-systems && cargo run

### Volume 3 Build Requirements

- Linux kernel 5.8+ (or WSL2 with kernel 5.15+)
- `sudo` access
- Nightly Rust with the `rust-src` component

```bash
# One-time setup
rustup install nightly
rustup component add rust-src --toolchain nightly

# Build the eBPF kernel probe (build `core` from source)
cd volume-3-kernel
cargo +nightly build -Z build-std=core --target bpfel-unknown-none --release -p sentinel-ebpf

# Build the userspace supervisor (stable is fine)
cargo build -p sentinel-core

# Run with root privileges
sudo ./target/debug/sentinel-core
```

> **Note:** The eBPF probe must be built before `sentinel-core` because
> `include_bytes_aligned!` embeds the probe binary at compile time.
> The build command uses `-Z build-std=core` to compile the `core` library
> from source – this is necessary because the `bpfel-unknown-none` target
> has no precompiled standard library.

```

---

### License

The code in `/volume-0-prelude` and `/volume-1-foundations` is released under
the [MIT License](LICENSE).

The code in `/volume-2-systems` and `/volume-3-kernel` is provided as
reference material for enrolled students. Redistribution outside of
personal study is not permitted.

---

📘 **Want the full learning experience?**
Visit [voltroniq.com/project-sentinel](https://voltroniq.com/project-sentinel)