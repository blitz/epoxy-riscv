//! This module implements the main application logic.

use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use failure::ResultExt;
use log::{debug, info};
use serde_dhall;
use std::path::Path;

use crate::cfgfile;
use crate::cfgtypes;
use crate::codegen;
use crate::kernel_codegen;
use crate::runtypes;

/// The virtual address in processes where mappings of resources start.
///
/// TODO This should be configurable, because it might conflict with the addresses at which the
/// binaries are linked.
const VIRT_RESOURCE_START: u64 = 0x40000000;

/// The default page size.
const PAGE_SIZE: u64 = 0x1000;

fn page_align_up(v: u64) -> u64 {
    (v + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

/// Take resource mappings and resolve them into named resources.
///
/// TODO This is needlessly long/unmodular/ugly.
fn to_process_resources(
    proc_name: &str,
    needs: &[cfgtypes::NamedResourceType],
    mappings: &[cfgtypes::Mapping],
    devices: &[cfgtypes::NamedResource],
) -> Result<runtypes::ResourceMap, Error> {
    let mut vstart = VIRT_RESOURCE_START;

    needs
        .iter()
        .map(|need| -> Result<(String, cfgtypes::Resource), Error> {
            // A needed resource "dev" for process "hello" means we need to look for a mapping to
            // "hello.dev".
            let mapping_to = proc_name.to_owned() + "." + &need.name;

            let source_name = mappings
                .iter()
                .find(|m| m.to == mapping_to)
                .map(|m| m.from.clone())
                .ok_or_else(|| {
                    format_err!("Failed to find mapping for needed resource {}", mapping_to)
                })?;

            let source_res = devices
                .iter()
                .find(|d| d.name == source_name)
                .ok_or_else(|| {
                    format_err!(
                        "Failed to find resource {} referenced from process {}",
                        source_name,
                        proc_name
                    )
                })?;

            info!("Mapping {} to {}", source_name, mapping_to);
            Ok((need.name.clone(), source_res.resource.clone()))
        })
        .map(|v| {
            v.map(|(name, res)| match res {
                cfgtypes::Resource::Framebuffer { format, region } => (
                    name,
                    runtypes::MemoryResource {
                        region: runtypes::VirtualMemoryRegion {
                            virt_start: {
                                let old = vstart;
                                vstart = page_align_up(vstart + region.size);
                                old
                            },
                            phys: region.clone(),
                        },
                        meta: runtypes::ResourceMetaInfo::Framebuffer { format: format },
                    },
                ),
            })
        })
        .collect()
}

fn internalize_process(
    root: &Path,
    machine: &cfgtypes::Machine,
    process: &cfgtypes::Process,
    mappings: &[cfgtypes::Mapping],
) -> Result<runtypes::Process, Error> {
    let app_cfg_file = cfgfile::find(cfgfile::Type::Application, root, &process.program);
    info!(
        "Using {} as configuration file for process {}",
        app_cfg_file.display(),
        process.name
    );

    let program: cfgtypes::Application = serde_dhall::from_file(app_cfg_file)
        .parse()
        .context("Failed to parse machine description")?;

    Ok(runtypes::Process {
        name: process.name.clone(),
        binary: program.binary,
        resources: to_process_resources(&process.name, &program.needs, mappings, &machine.devices)
            .context("Failed to resolve process resources for process")?,
    })
}

/// Take a system description as it comes in from the config files and read all other configurations
/// it refernces.
fn configure_system(
    root: &Path,
    system: &cfgtypes::System,
) -> Result<runtypes::Configuration, Error> {
    let machine: cfgtypes::Machine =
        serde_dhall::from_file(cfgfile::find(cfgfile::Type::Machine, root, &system.machine))
            .parse()
            .context("Failed to parse machine description")?;

    let processes: Vec<runtypes::Process> = system
        .processes
        .iter()
        .map(|p| internalize_process(root, &machine, p, &system.mappings))
        .collect::<Result<Vec<runtypes::Process>, Error>>()?;

    Ok(runtypes::Configuration {
        name: system.name.clone(),
        available_memory: machine.available_memory.clone(),
        processes: processes
            .into_iter()
            .map(|p| -> (String, runtypes::Process) { (p.name.clone(), p) })
            .collect(),
    })
}

fn epoxy_verify(system: &runtypes::Configuration) -> Result<(), Error> {
    info!("Everything is fine!");
    debug!("Resolved runtime configuration: {:#?}", system);

    Ok(())
}

fn epoxy_list_processes(system: &runtypes::Configuration) -> Result<(), Error> {
    for pname in system.processes.keys() {
        println!("{}", pname);
    }

    Ok(())
}

fn epoxy_configure_process(
    system: &runtypes::Configuration,
    pname: &str,
    lang: &str,
) -> Result<(), Error> {
    let process = system
        .processes
        .get(pname)
        .ok_or_else(|| format_err!("Failed to find processes {}", pname))?;

    print!(
        "{}",
        codegen::generate(lang.parse::<codegen::Language>()?, &process)
    );

    Ok(())
}

fn epoxy_configure_kernel(system: &runtypes::Configuration, do_header: bool) -> Result<(), Error> {
    print!(
        "{}",
        if do_header {
            kernel_codegen::generate_hpp(&system)?
        } else {
            kernel_codegen::generate_cpp(&system)?
        }
    );

    Ok(())
}

pub fn main() -> Result<(), Error> {
    let matches = App::new("Epoxy Harden System Configuration")
        .arg(Arg::with_name("verbosity")
             .short("v")
             .multiple(true)
             .help("Increase message verbosity"))
        .arg(Arg::with_name("quiet")
             .short("q")
             .help("Silence all output"))
        .arg(Arg::with_name("cfg-root")
             .short("r")
             .long("cfg-root")
             .value_name("CFGROOT")
             .required(true)
             .help("The directory where configuration files will be looked for."))
        .arg(Arg::with_name("system")
             .short("s")
             .long("system")
             .value_name("SYSTEM")
             .required(true)
             .help("The system name that should be used. This should match a Dhall file in CFGROOT/systems."))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("verify")
                    .about("Verify the system configuration"))
        .subcommand(SubCommand::with_name("list-processes")
                    .about("List all processes"))
        .subcommand(SubCommand::with_name("configure-process")
                    .about("Generate configuration code for one process")
                    .arg(Arg::with_name("process")
                         .value_name("PROC")
                         .required(true)
                         .help("The process to generate code for"))
                    .arg(Arg::with_name("language")
                         .short("l")
                         .long("language")
                         .value_name("LANG")
                         .default_value("c++")))
        .subcommand(SubCommand::with_name("configure-kernel")
                    .about("Generate configuration code for the kernel (C++ only)")
                    .arg(Arg::with_name("header")
                         .short("h")
                         .long("header")
                         .help("Generate the header instead of the C++ source")))
        .get_matches();

    let verbose = matches.occurrences_of("verbosity") as usize;
    let quiet = matches.is_present("quiet");

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .init()
        .unwrap();

    let cfg_root = Path::new(
        matches
            .value_of("cfg-root")
            .expect("required option missing"),
    );
    let cfg_system = cfgfile::find(
        cfgfile::Type::System,
        cfg_root,
        &matches.value_of("system").expect("required option missing"),
    );

    info!("Using system description at: {}", cfg_system.display());

    let system_spec: cfgtypes::System = serde_dhall::from_file(cfg_system)
        .parse()
        .context("Failed to parse system description")?;

    debug!("System description is: {:#?}", system_spec);

    let configured_system = configure_system(cfg_root, &system_spec)?;

    if let Some(_) = matches.subcommand_matches("verify") {
        epoxy_verify(&configured_system)
    } else if let Some(_) = matches.subcommand_matches("list-processes") {
        epoxy_list_processes(&configured_system)
    } else if let Some(cfg_proc_matches) = matches.subcommand_matches("configure-process") {
        epoxy_configure_process(
            &configured_system,
            cfg_proc_matches
                .value_of("process")
                .expect("required option missing"),
            cfg_proc_matches
                .value_of("language")
                .expect("option with default value missing"),
        )
    } else if let Some(cfg_kern_matches) = matches.subcommand_matches("configure-kernel") {
        epoxy_configure_kernel(&configured_system, cfg_kern_matches.is_present("header"))
    } else {
        Err(format_err!("Unknown subcommand"))
    }
}
