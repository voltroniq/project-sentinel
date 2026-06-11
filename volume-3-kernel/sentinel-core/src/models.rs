use serde::{Serialize, Deserialize};
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum AttackVector { NetworkTraffic, FileSystem, Unknown }

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatProfile {
    pub vector: AttackVector,
    pub target_port: u16,
    pub requires_admin: bool,
    pub signature: String,
    pub source_ip: Option<String>,
}

impl ThreatProfile {
    pub fn new(
        vector: AttackVector,
        target_port: u16,
        requires_admin: bool,
        signature: String,
        source_ip: Option<String>,
    ) -> Self {
        Self { vector, target_port, requires_admin, signature, source_ip }
    }

    pub fn is_critical(&self) -> bool {
        match self.vector {
            AttackVector::NetworkTraffic => self.target_port == 22 || self.target_port == 80,
            AttackVector::FileSystem     => self.requires_admin,
            AttackVector::Unknown        => false,
        }
    }
}

impl fmt::Display for ThreatProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = if self.is_critical() { "CRITICAL" } else { "STANDARD" };
        let ip_string = match &self.source_ip {
            Some(ip) => ip,
            None     => "UNKNOWN",
        };
        write!(
            f,
            "[{}] Vector: {:?} | Port: {} | Src: {} | Payload: \"{}\"",
            level, self.vector, self.target_port, ip_string, self.signature
        )
    }
}

#[derive(Debug)]
pub enum SentinelError {
    EmptyPayload,
    MalformedStructure,
    #[allow(dead_code)]
    InvalidPort(ParseIntError),
}