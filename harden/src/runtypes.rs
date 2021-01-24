//! The types that describe a configured system. These overlap with, but are not the same as
//! `cfgtypes`.

use std::collections::BTreeMap;

use crate::cfgtypes;
use crate::framebuffer;

#[derive(Debug)]

pub enum MemoryRegion {
    AnonymousZeroes { size: u64 },
    Phys { size: u64, start: u64 },
}

impl MemoryRegion {
    pub fn size(&self) -> u64 {
        match self {
            MemoryRegion::AnonymousZeroes { size } => *size,
            MemoryRegion::Phys { size, .. } => *size,
        }
    }
}

impl From<&cfgtypes::MemoryRegion> for MemoryRegion {
    fn from(cmem: &cfgtypes::MemoryRegion) -> Self {
        MemoryRegion::Phys {
            start: cmem.start,
            size: cmem.size,
        }
    }
}

/// A memory mapping in a process.
///
/// TODO: This also needs to model permissions.
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
        self.region.phys.size()
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

impl Process {
    pub fn initial_stack_pointer(&self) -> u64 {
        self.stack.region.virt_start + self.stack.size() - 8
    }
}

#[derive(Debug)]
pub struct Configuration {
    pub name: String,
    pub available_memory: Vec<cfgtypes::MemoryRegion>,
    pub processes: ProcessMap,
}
