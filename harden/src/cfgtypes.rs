//! This module contains types for data we read from configuration files. These data structures are
//! usually not meant to be modified.

use serde::Deserialize;

use crate::framebuffer;

#[derive(Deserialize, Debug, Clone)]
pub struct Process {
    pub name: String,
    pub program: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mapping {
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct System {
    pub name: String,
    pub machine: String,
    pub kernel: String,
    pub processes: Vec<Process>,
    pub mappings: Vec<Mapping>,
}

// TODO Use Interval for this.
#[derive(Deserialize, Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Framebuffer,
    SiFivePLIC,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Resource {
    /// A simple framebuffer.
    Framebuffer {
        format: framebuffer::Format,
        region: MemoryRegion,
    },
    /// A SiFive Platform-Level Interrupt Controller.
    SiFivePLIC {
        /// The number of supported external interrupts.
        ndev: u16,
        region: MemoryRegion,
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct NamedResource {
    pub name: String,
    pub resource: Resource,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NamedResourceType {
    pub name: String,
    pub r#type: ResourceType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub available_memory: Vec<MemoryRegion>,
    pub devices: Vec<NamedResource>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Application {
    pub name: String,
    pub needs: Vec<NamedResourceType>,
    pub binary: String,
}
