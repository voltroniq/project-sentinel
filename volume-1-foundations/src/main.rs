use std::io::{self, Write};
use std::num::ParseIntError;

// --- DOMAIN MODELS ---

#[derive(Debug)]
enum AttackVector { NetworkTraffic, FileSystem, Unknown }

#[derive(Debug)]
struct ThreatProfile {
    vector: AttackVector,
    target_port: u16,
    requires_admin: bool,
    signature: String,
}

// 1. OUR CUSTOM ERROR SYSTEM
// This replaces lazy string errors with programmatic, exact failure states.
#[derive(Debug)]
enum SentinelError {
    EmptyPayload,
    MalformedStructure,
    InvalidPort(ParseIntError), // Wraps Rust's native integer parsing error
}

// --- CORE LOGIC ---

// 2. Notice the return signature now uses our strict SentinelError
fn analyze_payload(payload: &str) -> Result<ThreatProfile, SentinelError> {
    let clean = payload.trim();
    if clean.is_empty() { return Err(SentinelError::EmptyPayload); }

    let mut parts = clean.split_whitespace();
    let command = parts.next().ok_or(SentinelError::MalformedStructure)?;

    // 3. Using Tuple Destructuring for a cleaner, more compact assignment
    let (vector, req_admin, default_port, expects_port) = match command {
        "nc" | "ssh" => (AttackVector::NetworkTraffic, false, 22, true),
        "rm" | "chmod" => (AttackVector::FileSystem, true, 0, false),
        _ => (AttackVector::Unknown, false, 0, false),
    };

    let target_port: u16 = if expects_port {
        match parts.next() {
            // Because target_port is strictly defined as u16, Rust infers the parse type automatically!
            Some(port_str) => port_str.parse().map_err(SentinelError::InvalidPort)?,
            None => default_port,
        }
    } else {
        default_port
    };

    Ok(ThreatProfile {
        vector,
        target_port,
        requires_admin: req_admin,
        signature: clean.to_string(),
    })
}

// --- EXECUTION ENGINE ---

fn main() -> Result<(), io::Error> {
    println!("=== PROJECT SENTINEL TELEMETRY MATRIX ===");
    
    // 4. Initialize our dynamic, heap-allocated Vector matrix
    let mut threat_ledger: Vec<ThreatProfile> = Vec::new();

    loop {
        print!("sentinel_engine > ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let query = buffer.trim();

        // Graceful shutdown sequence
        if query == "exit" || query == "quit" {
            println!("Shutting down Sentinel Telemetry Matrix...");
            break;
        }

        if query == "history" {
            println!("\n--- ACTIVE SECURITY LEDGER ---");
            for (index, profile) in threat_ledger.iter().enumerate() {
                println!("[{}] Vector: {:?} | Port: {} | Admin: {} | CMD: {}", 
                    index, profile.vector, profile.target_port, profile.requires_admin, profile.signature);
            }
            println!("------------------------------\n");
            continue;
        }

        match analyze_payload(query) {
            Ok(profile) => {
                println!("Alert: Threat Profile Generated. Appending to ledger.");
                // 5. Move ownership of the profile into our vector memory matrix
                threat_ledger.push(profile); 
            }
            Err(e) => {
                // 6. Catch exact failure states gracefully without panicking
                match e {
                    SentinelError::EmptyPayload => println!("Notice: Blank input ignored."),
                    SentinelError::MalformedStructure => println!("Error: Command unreadable."),
                    SentinelError::InvalidPort(err) => println!("Error: Network port invalid. ({})", err),
                }
            }
        }
    }
    
    Ok(())
}