# Project Sentinel | Official Companion Repository

**Master Systems Programming in Rust.**

Project Sentinel is a 17-chapter Rust systems programming curriculum that progresses from basic CLI parsing to advanced Linux kernel observability. This repository serves as the official companion for the [Project Sentinel](https://www.voltroniq.com/project-sentinel) course.

### How to use this repository
This repository contains the completed, working codebases for the course.
- **Volume 0 (Preludes 1-4)** is fully open-source and available here for learning.
- **Volume 1 (Chapters 1-4)** is fully open-source and available here for learning.
- **Volume 2 & 3** provide reference code accompanying the course material.

*Note: For the pedagogical breakdown and the step-by-step guidance, please join the full course at [Project Sentinel](https://www.voltroniq.com/project-sentinel)*

---

### Folder Structure

#### `/volume-0-prelude`
Single file with commented examples for all four preludes.
* `src/main.rs`
* **Status:** 100% Complete.

#### `/volume-1-foundations`
Fully functional code for the CLI Command Scanner.
* `src/main.rs`
* `src/parser.rs`
* **Status:** 100% Complete.

#### `/volume-2-systems`
Example implementations of the Module & Threading architecture.
* `src/engine.rs`
* `src/sandbox.rs`
* **Status:** Reference implementations. 

#### `/volume-3-kernel`
Advanced systems integration with Linux Namespaces and eBPF.
* **Status:** [Premium Content]
* To keep the premium curriculum self-contained, portions of the kernel instrumentation and namespace isolation implementation have been abstracted from the public repository.
* *To unlock the full source code for the kernel-level supervisor, [enroll in the course](https://www.voltroniq.com/project-sentinel).*

---

### System Prerequisites
To build the code in this repository:
1. **Rust Toolchain:** `rustup` (latest stable).
2. **Linux Environment:** Required for Volume 3 features (Namespaces/eBPF). 
3. **Dependencies:** `cargo build` will automatically pull required crates (Tokio, Aya, Serde).

---

# Volume 2

```bash
cd volume-2-systems
cargo run
```

# Volume 3 (Linux only — WSL supported)

```bash
cd volume-3-kernel

# Build the eBPF probe
cargo build --release --target bpfel-unknown-none -p sentinel-ebpf

# Build the main supervisor
cargo build -p sentinel-core

# Run with root privileges
sudo ./target/debug/sentinel-core
```
---

## License

-   **Volume 0 & 1** – MIT License (free and open source).
    
-   **Volumes 2 & 3** – Proprietary to Voltroniq. The source code is provided as reference material for enrolled students. Unauthorised distribution is prohibited.
 

📘 **Want the full learning experience?**  
Visit [voltroniq.com/project-sentinel](https://voltroniq.com/project-sentinel) for the complete course, resources and step‑by‑step guidance.