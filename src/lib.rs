// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-printer-discovery: printer discovery component model.
//!
//! Real implementation: discoverys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Discovery {
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
pub enum PrinterDiscoveryError {
    EmptyDiscoveryName,
    DuplicateDiscovery(String),
    EmptyEventSubject,
}

impl fmt::Display for PrinterDiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDiscoveryName => write!(f, "discovery name must not be empty"),
            Self::DuplicateDiscovery(id) => write!(f, "duplicate discovery {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PrinterDiscoveryError {}

pub struct PrinterDiscovery {
    pub name: String,
    discoverys: BTreeMap<String, Discovery>,
    events: BTreeMap<String, Event>,
}

impl PrinterDiscovery {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            discoverys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_discovery(&mut self, item: Discovery) -> Result<(), PrinterDiscoveryError> {
        if item.name.trim().is_empty() {
            return Err(PrinterDiscoveryError::EmptyDiscoveryName);
        }
        let id = item.id.clone();
        if self.discoverys.contains_key(&id) {
            return Err(PrinterDiscoveryError::DuplicateDiscovery(id.clone()));
        }
        self.discoverys.insert(id, item);
        Ok(())
    }

    pub fn discovery_count(&self) -> usize {
        self.discoverys.len()
    }

    pub fn get_discovery(&self, id: &str) -> Option<&Discovery> {
        self.discoverys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PrinterDiscoveryError> {
        if event.subject.trim().is_empty() {
            return Err(PrinterDiscoveryError::EmptyEventSubject);
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

    fn sample_discovery(id: &str, name: &str) -> Discovery {
        Discovery {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = PrinterDiscovery::new("PrinterDiscovery");
        assert_eq!(m.name, "PrinterDiscovery");
        assert_eq!(m.discovery_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_discovery() {
        let mut m = PrinterDiscovery::new("PrinterDiscovery");
        m.add_discovery(sample_discovery("a1", "Alpha")).unwrap();
        assert_eq!(m.discovery_count(), 1);
        assert!(m.get_discovery("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = PrinterDiscovery::new("PrinterDiscovery");
        assert!(m.add_discovery(sample_discovery("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = PrinterDiscovery::new("PrinterDiscovery");
        m.add_discovery(sample_discovery("x", "One")).unwrap();
        assert!(m.add_discovery(sample_discovery("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = PrinterDiscovery::new("PrinterDiscovery");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
