// PROJECT SENTINEL — Volume 0: Simulation Core
// Companion code for voltroniq.com/project-sentinel
//
// Run each prelude by uncommenting the relevant block and running: cargo run

// ── PRELUDE 1: Diagnostic Boot ──────────────────────────────────────────────
fn main() {
    println!("Simulation pod online.");

    // 'mut' allows the variable to be changed after declaration
    let mut threat_level = 1;
    threat_level += 1;

    println!("Current simulated threat level: {}", threat_level);
}

// ── PRELUDE 2: The Sorting Gate ──────────────────────────────────────────────
// Uncomment the block below, comment out Prelude 1's main(), and run: cargo run
//
// enum Severity { Low, Critical }
//
// fn main() {
//     let mock_log = Severity::Critical;
//     match mock_log {
//         Severity::Low      => println!("Log ignored."),
//         Severity::Critical => println!("SYSTEM ALERT: Locking down simulation pod!"),
//     }
// }

// ── PRELUDE 3: Safe Passage ──────────────────────────────────────────────────
// fn scan_log(log: &str) {
//     println!("Scanning reference: {}", log);
// }
//
// fn main() {
//     let dummy_log = String::from("Network spike detected.");
//     scan_log(&dummy_log);
//     println!("Log intact on main thread: {}", dummy_log);
// }

// ── PRELUDE 4: Handling the Unknown ─────────────────────────────────────────
// fn extract_ip(id: u32) -> Option<String> {
//     if id == 1 { Some(String::from("192.168.1.50")) } else { None }
// }
//
// fn main() {
//     let ip = extract_ip(2).unwrap_or(String::from("0.0.0.0"));
//     println!("Target IP: {}", ip);
// }