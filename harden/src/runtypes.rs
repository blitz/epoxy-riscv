//! The types that describe a configured system. These overlap with, but are not the same as
//! `cfgtypes`.

use std::collections::BTreeMap;

use crate::cfgtypes;
use crate::framebuffer;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct VirtualMemoryRegion {
    pub virt_start: u64,
    pub phys: MemoryRegion,
}

impl VirtualMemoryRegion {
    pub fn size(&self) -> u64 {
        self.phys.size()
    }
}

#[derive(Debug)]
pub enum ResourceMetaInfo {
    Framebuffer { format: framebuffer::Format },
    SifivePlic { ndev: u16 },
    SBITimer { freq_hz: u64 },
}

/// A system resource. This will be a bunch of meta information with an optional memory mapping.
#[derive(Debug)]
pub struct Resource {
    pub meta: ResourceMetaInfo,
    pub opt_region: Option<VirtualMemoryRegion>,
}

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

    /// Additional anonymous memory regions (stack, heap, ...).
    pub anon_mem: Vec<VirtualMemoryRegion>,

    pub stack_ptr: u64,
    pub heap_start: u64,
    pub heap_end: u64,
}

#[derive(Debug)]
pub struct Configuration {
    pub name: String,
    pub available_memory: Vec<cfgtypes::MemoryRegion>,
    pub kernel: Process,
    pub processes: ProcessMap,
}
