//! This module implements the main application logic.

use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use failure::ResultExt;
use log::{debug, info};
use serde_dhall;
use std::path::Path;

use crate::boot_image;
use crate::bump_ptr_alloc::{BumpPointerAlloc, SimpleAlloc};
use crate::cfgfile;
use crate::cfgtypes;
use crate::codegen;
use crate::constants::*;
use crate::interval::Interval;
use crate::kernel_codegen;
use crate::runtypes;

/// Flatten a nested result.
///
/// TODO: Nightly has a `.flatten()` method that replaces this function.
fn rflatten<T>(r: Result<Result<T, Error>, Error>) -> Result<T, Error> {
    match r {
        Ok(v) => v,
        Err(e) => Err(e),
    }
}

/// Lift an option out of a result.
fn option_lift<T>(r: Result<Option<T>, Error>) -> Option<Result<T, Error>> {
    match r {
        Ok(Some(v)) => Some(Ok(v)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

fn make_user_stack<T: SimpleAlloc>(valloc: &mut T) -> Result<runtypes::MemoryResource, Error> {
    valloc
        .alloc(PAGE_SIZE)
        .ok_or_else(|| format_err!("Failed to allocate stack guard page"))?;

    let stack = runtypes::MemoryResource {
        region: runtypes::VirtualMemoryRegion {
            virt_start: valloc
                .alloc(USER_STACK_SIZE)
                .ok_or_else(|| format_err!("Failed to allocate stack"))?,
            phys: runtypes::MemoryRegion::AnonymousZeroes {
                size: USER_STACK_SIZE,
            },
        },
        meta: runtypes::ResourceMetaInfo::Stack,
    };

    valloc
        .alloc(PAGE_SIZE)
        .ok_or_else(|| format_err!("Failed to allocate stack guard page"))?;

    Ok(stack)
}

fn map_memory<T: SimpleAlloc>(
    valloc: &mut T,
    region: &cfgtypes::MemoryRegion,
) -> Result<runtypes::VirtualMemoryRegion, Error> {
    Ok(runtypes::VirtualMemoryRegion {
        virt_start: valloc.alloc(region.size).ok_or_else(|| {
            format_err!(
                "Failed to allocate virtual memory memory region {:#?}",
                region
            )
        })?,
        phys: runtypes::MemoryRegion::from(region),
    })
}

/// Map a resource into memory. This might return None, if the resource does not need to be mapped.
fn map_resource<T: SimpleAlloc>(
    valloc: &mut T,
    device: &cfgtypes::Resource,
) -> Result<Option<runtypes::MemoryResource>, Error> {
    Ok(match device {
        cfgtypes::Resource::SiFivePLIC { ndev, region } => Some(runtypes::MemoryResource {
            region: map_memory(valloc, region)?,
            meta: runtypes::ResourceMetaInfo::SifivePlic { ndev: *ndev },
        }),
        cfgtypes::Resource::Framebuffer { format, region } => Some(runtypes::MemoryResource {
            region: map_memory(valloc, region)?,
            meta: runtypes::ResourceMetaInfo::Framebuffer {
                format: format.clone(),
            },
        }),
        cfgtypes::Resource::SBITimer { .. } => None,
    })
}

/// Take resource mappings and resolve them into named resources.
///
/// TODO This is needlessly long/unmodular/ugly.
fn to_process_resources<T: SimpleAlloc>(
    valloc: &mut T,
    proc_name: &str,
    needs: &[cfgtypes::NamedResourceType],
    mappings: &[cfgtypes::Mapping],
    devices: &[cfgtypes::NamedResource],
) -> Result<runtypes::ResourceMap, Error> {
    let rmap: runtypes::ResourceMap = needs
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
        .map(
            |v| -> Result<Option<(String, runtypes::MemoryResource)>, Error> {
                rflatten(v.map(
                    |(name, dev)| -> Result<Option<(String, runtypes::MemoryResource)>, Error> {
                        Ok(map_resource(valloc, &dev)?.map(|r| (name, r)))
                    },
                ))
            },
        )
        .filter_map(|v| option_lift(v))
        .collect::<Result<runtypes::ResourceMap, Error>>()?;

    Ok(rmap)
}

#[derive(Debug, Clone, Copy)]
enum ProcessType {
    Kernel,
    User,
}

fn get_process_valloc(process_type: ProcessType) -> impl SimpleAlloc {
    BumpPointerAlloc::new(
        match process_type {
            ProcessType::User => Interval {
                from: USER_RESOURCE_START,
                to: USER_RESOURCE_END,
            },
            ProcessType::Kernel => Interval {
                from: KERN_RESOURCE_START,
                to: KERN_RESOURCE_END,
            },
        },
        PAGE_SIZE,
    )
}

fn internalize_process(
    root: &Path,
    machine: &cfgtypes::Machine,
    process: &cfgtypes::Process,
    mappings: &[cfgtypes::Mapping],
    process_type: ProcessType,
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

    let mut valloc = get_process_valloc(process_type);

    Ok(runtypes::Process {
        name: process.name.clone(),
        binary: program.binary,
        stack: match process_type {
            ProcessType::User => Some(make_user_stack(&mut valloc)?),

            // The kernel takes care of its own stack.
            ProcessType::Kernel => None,
        },
        resources: to_process_resources(
            &mut valloc,
            &process.name,
            &program.needs,
            mappings,
            &machine.devices,
        )
        .context("Failed to resolve process resources for process")?,
    })
}

/// Take a system description as it comes in from the config files and read all other configurations
/// it references.
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
        .map(|p| internalize_process(root, &machine, p, &system.mappings, ProcessType::User))
        .collect::<Result<Vec<runtypes::Process>, Error>>()?;

    Ok(runtypes::Configuration {
        name: system.name.clone(),
        available_memory: machine.available_memory.clone(),
        kernel: internalize_process(
            root,
            &machine,
            &cfgtypes::Process {
                name: system.kernel.clone(),
                program: system.kernel.clone(),
            },
            &system.mappings,
            ProcessType::Kernel,
        )?,
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

fn epoxy_configure_kernel(
    system: &runtypes::Configuration,
    out_type: &str,
    user_root: &Path,
) -> Result<(), Error> {
    print!(
        "{}",
        match out_type {
            "state-hpp" => kernel_codegen::generate_hpp(&system)?,
            "state-cpp" => kernel_codegen::generate_cpp(&system, user_root)?,
            "resources" => codegen::generate(codegen::Language::CPP, &system.kernel),
            _ => Err(format_err!(
                "Unrecognized output type. Should be one of: state-hpp state-cpp resources"
            ))?,
        }
    );

    Ok(())
}

fn epoxy_boot_image(system: &runtypes::Configuration, user_binaries: &Path) -> Result<(), Error> {
    boot_image::generate(system, user_binaries)
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
                    .arg(Arg::with_name("type")
                         .required(true)
                         .help("Specify what type of output should be generated: state-hpp, state-cpp, or resources"))
                    .arg(Arg::with_name("user-binaries")
                         .required(true)
                         .help("The path where user binaries can be found"))
                    )
        .subcommand(SubCommand::with_name("boot-image")
                    .about("Generate a bootable image for the target platform")
                    .arg(Arg::with_name("user-binaries")
                         .required(true)
                         .help("The path where user binaries can be found")))
        .get_matches();

    let verbose = matches.occurrences_of("verbosity") as usize;
    let quiet = matches.is_present("quiet");

    stderrlog::new()
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

    debug!("Configured system is: {:#x?}", configured_system);

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
        epoxy_configure_kernel(
            &configured_system,
            cfg_kern_matches
                .value_of("type")
                .expect("required option missing"),
            Path::new(
                cfg_kern_matches
                    .value_of("user-binaries")
                    .expect("required option missing"),
            ),
        )
    } else if let Some(boot_image_matches) = matches.subcommand_matches("boot-image") {
        epoxy_boot_image(
            &configured_system,
            Path::new(
                boot_image_matches
                    .value_of("user-binaries")
                    .expect("required option missing"),
            ),
        )
    } else {
        Err(format_err!("Unknown subcommand"))
    }
}
