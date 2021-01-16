//! The types that describe a configured system. These overlap with, but are not the same as
//! `cfgtypes`.

use std::collections::BTreeMap;

// Re-export config types that are re-used.
pub use crate::cfgtypes::MemoryRegion;
use crate::framebuffer;

/// A memory mapping in a process.
#[derive(Debug)]
pub struct VirtualMemoryRegion {
    pub virt_start: u64,
    pub phys: MemoryRegion,
}

#[derive(Debug)]
pub enum ResourceMetaInfo {
    Stack,
    Framebuffer { format: framebuffer::Format },
}

/// A system resource that is represented as a memory mapping. This is basically a piece of mapped
/// physical memory plus some metainformation.
#[derive(Debug)]
pub struct MemoryResource {
    pub region: VirtualMemoryRegion,

    /// Metainformation about this resource.
    pub meta: ResourceMetaInfo,
}

impl MemoryResource {
    pub fn size(&self) -> u64 {
        self.region.phys.size
    }
}

pub type ProcessMap = BTreeMap<String, Process>;
pub type ResourceMap = BTreeMap<String, MemoryResource>;

/// A process with its binary and assigned resources.
#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub binary: String,

    /// A mapping from resource name (the one specified as `needs` in the application description)
    /// to an actual resource.
    pub resources: ResourceMap,

    /// The stack of the single thread in the process.
    pub stack: MemoryResource,
}

#[derive(Debug)]
pub struct Configuration {
    pub name: String,
    pub available_memory: Vec<MemoryRegion>,
    pub processes: ProcessMap,
}
