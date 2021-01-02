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
