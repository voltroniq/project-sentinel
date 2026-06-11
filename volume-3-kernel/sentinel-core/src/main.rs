use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncBufReadExt, BufReader, AsyncWriteExt};
use aya::{Bpf, include_bytes_aligned};
use aya::programs::KProbe;
use aya::maps::perf::PerfEventArray;
use aya::util::online_cpus;
use bytes::BytesMut;

mod models;
mod engine;
mod sandbox;

use engine::analyze_payload;
use sandbox::ProcessSupervisor;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SystemEvent {
    pub pid: u32,
    pub event_type: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PROJECT SENTINEL EBPF SUPERVISOR ===");

    // Load the eBPF program
    let mut bpf = Bpf::load(include_bytes_aligned!("../../target/bpfel-unknown-none/release/sentinel-ebpf"))?;

    // Print all available program names (debug)
    println!("[EBPF] Available programs:");
    for (name, _) in bpf.programs() {
        println!("  - {}", name);
    }

    // Try to get the kprobe program (the name from debug output is "kprobe")
    let prog = if let Some(p) = bpf.program_mut("kprobe") {
        p
    } else {
        panic!("No kprobe program found. Available: {:?}", 
            bpf.programs().map(|(n,_)| n).collect::<Vec<_>>());
    };

    // Attach the kprobe to sys_execve
    let program: &mut KProbe = prog.try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;
    println!("[EBPF] KProbe attached to sys_execve.");

    // Read events from the perf event array (synchronous, no .await)
    let mut telemetry_stream = PerfEventArray::try_from(bpf.map_mut("TELEMETRY_STREAM").unwrap())?;
    for cpu_id in online_cpus()? {
        let mut buf = telemetry_stream.open(cpu_id, None)?;
        tokio::spawn(async move {
            let mut buffers = vec![BytesMut::with_capacity(1024); 10];
            loop {
                // read_events is synchronous in aya 0.11 → remove .await
                let events = buf.read_events(&mut buffers).unwrap();
                for i in 0..events.read {
                    let event_bytes = &buffers[i];
                    let event = unsafe {
                        std::ptr::read_unaligned(event_bytes.as_ptr() as *const SystemEvent)
                    };
                    println!("\n[GLOBAL ALERT] Unsandboxed Process Execution Detected! PID: {}", event.pid);
                }
            }
        });
    }

    // Async REPL (same as before)
    let threat_ledger = Arc::new(Mutex::new(Vec::new()));
    let mut execution_counter = 0u32;
    let mut stdin_reader = BufReader::new(io::stdin());
    let mut stdout_writer = io::stdout();

    loop {
        stdout_writer.write_all(b"\nsentinel_kernel > ").await?;
        stdout_writer.flush().await?;
        let mut buffer = String::new();
        stdin_reader.read_line(&mut buffer).await?;
        let query = buffer.trim();
        if query.is_empty() { continue; }

        let mut tokens = query.split_whitespace();
        let primary_command = tokens.next().unwrap();
        let arguments: Vec<String> = tokens.map(|s| s.to_string()).collect();

        if let Ok(profile) = analyze_payload(query) {
            let mut lock = threat_ledger.lock().unwrap();
            lock.push(profile);
        }

        execution_counter += 1;
        let log_name = format!("audit_task_{:03}.log", execution_counter);
        let supervisor = ProcessSupervisor::new(primary_command, arguments);
        match supervisor.execute_isolated(&log_name) {
            Ok(mut child) => {
                let pid = child.id().unwrap();
                println!("[KERNEL] Async container PID: {}", pid);
                tokio::spawn(async move {
                    if let Ok(status) = child.wait().await {
                        println!("\n[ASYNC REAPER] Container {} terminated: {}", pid, status);
                    }
                });
            }
            Err(e) => println!("[SYSTEM FAULT] {}", e),
        }
    }
}