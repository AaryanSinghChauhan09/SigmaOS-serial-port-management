// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-serial-port-management: serial port management component model.
//!
//! Real implementation: ports, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerialPortManagementError {
    EmptyPortName,
    DuplicatePort(String),
    EmptyEventSubject,
}

impl fmt::Display for SerialPortManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPortName => write!(f, "port name must not be empty"),
            Self::DuplicatePort(id) => write!(f, "duplicate port {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SerialPortManagementError {}

pub struct SerialPortManagement {
    pub name: String,
    ports: BTreeMap<String, Port>,
    events: BTreeMap<String, Event>,
}

impl SerialPortManagement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ports: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_port(&mut self, item: Port) -> Result<(), SerialPortManagementError> {
        if item.name.trim().is_empty() {
            return Err(SerialPortManagementError::EmptyPortName);
        }
        let id = item.id.clone();
        if self.ports.contains_key(&id) {
            return Err(SerialPortManagementError::DuplicatePort(id.clone()));
        }
        self.ports.insert(id, item);
        Ok(())
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }

    pub fn get_port(&self, id: &str) -> Option<&Port> {
        self.ports.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SerialPortManagementError> {
        if event.subject.trim().is_empty() {
            return Err(SerialPortManagementError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_port(id: &str, name: &str) -> Port {
        Port {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SerialPortManagement::new("SerialPortManagement");
        assert_eq!(m.name, "SerialPortManagement");
        assert_eq!(m.port_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_port() {
        let mut m = SerialPortManagement::new("SerialPortManagement");
        m.add_port(sample_port("a1", "Alpha")).unwrap();
        assert_eq!(m.port_count(), 1);
        assert!(m.get_port("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SerialPortManagement::new("SerialPortManagement");
        assert!(m.add_port(sample_port("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SerialPortManagement::new("SerialPortManagement");
        m.add_port(sample_port("x", "One")).unwrap();
        assert!(m.add_port(sample_port("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SerialPortManagement::new("SerialPortManagement");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
