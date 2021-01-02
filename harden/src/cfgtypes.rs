//! This module contains types for data we read from configuration files. These data structures are
//! usually not meant to be modified.

use serde::Deserialize;

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
    pub processes: Vec<Process>,
    pub mappings: Vec<Mapping>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ResourceType {
    Framebuffer,
}

#[derive(Deserialize, Debug, Clone)]
pub enum PixelFormat {
    R5G6B5,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Resource {
    Framebuffer {
        height: u32,
        width: u32,
        stride: u32,
        format: PixelFormat,
        region: MemoryRegion,
    },
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
