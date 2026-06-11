use crate::models::{ThreatProfile, SentinelError, AttackVector};

pub fn analyze_payload(payload: &str) -> Result<ThreatProfile, SentinelError> {
    let clean = payload.trim();
    if clean.is_empty() { return Err(SentinelError::EmptyPayload); }

    let mut parts = clean.split_whitespace();
    let command = parts.next().ok_or(SentinelError::MalformedStructure)?;

    let (vector, req_admin, default_port, expects_port) = match command {
        "nc" | "ssh" => (AttackVector::NetworkTraffic, false, 22, true),
        "rm" | "chmod" => (AttackVector::FileSystem, true, 0, false),
        _ => (AttackVector::Unknown, false, 0, false),
    };

    let target_port: u16 = if expects_port {
        match parts.next() {
            Some(port_str) => port_str.parse().map_err(SentinelError::InvalidPort)?,
            None => default_port,
        }
    } else {
        default_port
    };

    let source_ip: Option<String> = parts.next().map(|s| s.to_string());

    Ok(ThreatProfile::new(vector, target_port, req_admin, clean.to_string(), source_ip))
}