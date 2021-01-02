//! This module contains types for data we read from configuration files. These data structures are
//! usually not meant to be modified.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Process {
    proc_name: String,
    program: String,
}

#[derive(Deserialize, Debug)]
pub struct Mapping {
    from: String,
    to: String,
}

#[derive(Deserialize, Debug)]
pub struct System {
    name: String,
    machine: String,
    processes: Vec<Process>,
    mappings: Vec<Mapping>,
}

#[derive(Deserialize, Debug)]
pub struct MemoryRegion {
    start: u64,
    size: u64,
}

#[derive(Deserialize, Debug)]
pub enum ResourceType {
    Framebuffer,
}

#[derive(Deserialize, Debug)]
pub enum PixelFormat {
    R5G6B5,
}

#[derive(Deserialize, Debug)]
pub enum Resource {
    Framebuffer {
        height: u32,
        width: u32,
        stride: u32,
        format: PixelFormat,
        region: MemoryRegion,
    },
}

#[derive(Deserialize, Debug)]
pub struct NamedResource {
    name: String,
    resource: Resource,
}

#[derive(Deserialize, Debug)]
pub struct NamedResourceType {
    name: String,
    r#type: ResourceType,
}

#[derive(Deserialize, Debug)]
pub struct Machine {
    name: String,
    available_memory: Vec<MemoryRegion>,
    devices: Vec<NamedResource>,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    name: String,
    needs: Vec<NamedResourceType>,
    program: String,
}
