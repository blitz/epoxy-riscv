//! The types that describe a configured system. These overlap with, but are not the same as
//! `cfgtypes`.

use std::collections::BTreeMap;

// Re-export config types that are re-used.
pub use crate::cfgtypes::{MemoryRegion, Resource};

pub type ProcessMap = BTreeMap<String, Process>;
pub type ResourceMap = BTreeMap<String, Resource>;

/// A process with its binary and assigned resources.
#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub binary: String,

    /// A mapping from resource name (the one specified as `needs` in the application description)
    /// to an actual resource.
    pub resources: ResourceMap,
}

#[derive(Debug)]
pub struct Configuration {
    pub name: String,
    pub available_memory: Vec<MemoryRegion>,
    pub processes: ProcessMap,
}
