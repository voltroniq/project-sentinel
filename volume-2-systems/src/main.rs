use std::io::{self, Write};
use std::fs::File;
// 1. IMPORT SYNCHRONIZATION AND CONCURRENCY LIBRARIES
use std::thread;
use std::sync::{Arc, Mutex};

mod models;
mod engine;

use models::ThreatProfile;
use engine::analyze_payload;

const STORAGE_PATH: &str = "ledger.json";

fn save_ledger(ledger: &Vec<ThreatProfile>) -> Result<(), io::Error> {
    let file = File::create(STORAGE_PATH)?;
    serde_json::to_writer_pretty(file, ledger)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    println!("=== PROJECT SENTINEL CONCURRENT MATRIX ===");
    
    // 2. THE THREAD-SAFE VAULT
    // The Vec is protected by a Mutex so only one thread can
    // access or modify it at a time.
    //
    // The Mutex is wrapped in an Arc (Atomic Reference Counted pointer) 
    // so many threads can share ownership of the same ledger.
    let threat_ledger = Arc::new(Mutex::new(Vec::<ThreatProfile>::new()));

    loop {
        print!("sentinel_engine > ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let query = buffer.trim();

        // Graceful shutdown
        if query == "exit" || query == "quit" {
            println!("Shutting down Sentinel Concurrent Matrix...");
            return Ok(());
        }

        if query == "history" {
            println!("\n--- ACTIVE SECURITY LEDGER ---");
            // 3. ACQUIRING THE LOCK TO READ
            let ledger_lock = threat_ledger.lock().unwrap();
            for (index, profile) in ledger_lock.iter().enumerate() {
                println!("[{:02}] {}", index, profile);
            }
            println!("------------------------------\n");
            continue;
        }

        if query == "critical" {
            println!("\n--- CRITICAL FORENSIC REPORT ---");
            let ledger_lock = threat_ledger.lock().unwrap();
            let critical_threats: Vec<&ThreatProfile> = ledger_lock
                .iter()
                .filter(|profile| profile.is_critical())
                .collect();

            if critical_threats.is_empty() {
                println!("No high-level security compromise signatures identified inside vector.");
            } else {
                for (index, threat) in critical_threats.iter().enumerate() {
                    println!("[ALERT {:02}] {}", index, threat);
                }
            }
            println!("--------------------------------\n");
            continue;
        }

        if query == "save" {
            let ledger_lock = threat_ledger.lock().unwrap();
            // Rust automatically coerces &MutexGuard<Vec<T>> to &Vec<T> via the Deref trait.
            match save_ledger(&ledger_lock) {
                Ok(_) => println!("Success: Telemetry matrix safely synced to physical drive [{}].", STORAGE_PATH),
                Err(e) => println!("System Fault: Failed to write to disk. Error: {}", e),
            }
            continue;
        }

        // 4. CONCURRENT DISPATCH
        // Clone the Arc pointer so the background thread gets its own ownership token.
        // Clone the buffer string so the thread doesn't borrow data from the loop.
        let thread_ledger = Arc::clone(&threat_ledger);
        let payload = buffer.clone();

        // Spawn a background OS thread with a closure that takes ownership (move)
        thread::spawn(move || {
            match analyze_payload(&payload) {
                Ok(profile) => {
                    // Background thread requests lock, pushes data, and releases automatically
                    let mut ledger_lock = thread_ledger.lock().unwrap();
                    ledger_lock.push(profile);
                    
                    println!("\n[ASYNC ALERT] Threat Profile Processed in Background.");
                }
                Err(e) => {
                    // We explicitly log the error to the terminal without crashing the main loop
                    eprintln!("\n[ASYNC FAULT] Background parsing failed: {:?}", e);
                }
            }
        });
    }
}