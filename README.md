# Project Sentinel | Official Companion Repository

**Master Systems Programming in Rust.**

Project Sentinel is a 17-chapter curriculum that moves from basic CLI parsing to advanced Linux Kernel observability. This repository serves as the official companion for the [Voltroniq.com](https://voltroniq.com) course.

### How to use this repository
This repository contains the completed, working codebases for the course. 
- **Volume 1 (Chapters 1-4)** is fully open-source and available here for learning.
- **Volume 2 & 3** code snippets are provided here for reference. 

*Note: For the pedagogical breakdown, system architecture diagrams, and the step-by-step guidance, please join the full course at [Voltroniq.com](https://voltroniq.com).*

---

### Folder Structure

#### `/volume-1-basics`
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
* To prevent unauthorized distribution, the core logic for the eBPF KProbe injection and Namespace isolation has been abstracted. 
* *To unlock the full source code for the kernel-level supervisor, [enroll in the course](https://voltroniq.com).*

---

### System Prerequisites
To build the code in this repository:
1. **Rust Toolchain:** `rustup` (latest stable).
2. **Linux Environment:** Required for Volume 3 features (Namespaces/eBPF). 
3. **Dependencies:** `cargo build` will automatically pull required crates (Tokio, Aya, Serde).

---

### License
The code in `/volume-1-basics` is licensed under MIT. All other course material and architecture implementations are proprietary to Voltroniq.

*Ready to go deeper? [Start the journey at Voltroniq.com](https://voltroniq.com).*