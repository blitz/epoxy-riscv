#[macro_use]
extern crate failure;

use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use failure::ResultExt;
use log::{debug, error, info};
use serde_dhall;
use std::collections::BTreeMap;
use std::path::Path;

mod cfgfile;
mod cfgtypes;

type ResourceMap = BTreeMap<String, cfgtypes::Resource>;

#[derive(Debug)]
struct AssignedProcess {
    name: String,
    binary: String,

    /// A mapping from resource name (the one specified as `needs` in the application description)
    /// to an actual resource.
    resources: ResourceMap,
}

#[derive(Debug)]
struct RuntimeConfiguration {
    name: String,
    available_memory: Vec<cfgtypes::MemoryRegion>,
    processes: Vec<AssignedProcess>,
}

/// Take resource mappings and resolve them into named resources.
fn to_process_resources(
    proc_name: &str,
    needs: &[cfgtypes::NamedResourceType],
    mappings: &[cfgtypes::Mapping],
    devices: &[cfgtypes::NamedResource],
) -> Result<ResourceMap, Error> {
    needs
        .iter()
        .map(|need| -> Result<(String, cfgtypes::Resource), Error> {
            // A needed resource dev for process hello means we need to look for a mapping to
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
        .collect()
}

fn internalize_process(
    root: &Path,
    machine : &cfgtypes::Machine,
    process: &cfgtypes::Process,
    mappings: &[cfgtypes::Mapping],
) -> Result<AssignedProcess, Error> {
    let app_cfg_file = cfgfile::find(cfgfile::Type::Application, root, &process.program);
    info!(
        "Using {} as configuration file for process {}",
        app_cfg_file.display(),
        process.name
    );

    let program: cfgtypes::Application = serde_dhall::from_file(app_cfg_file)
        .parse()
        .context("Failed to parse machine description")?;

    Ok(AssignedProcess {
        name: process.name.clone(),
        binary: program.binary,
        resources: to_process_resources(&process.name, &program.needs, mappings, &machine.devices)
            .context("Failed to resolve process resources for process")?,
    })
}

/// Take a system description as it comes in from the config files and read all other configurations
/// it refernces.
fn configure_system(root: &Path, system: &cfgtypes::System) -> Result<RuntimeConfiguration, Error> {
    let machine: cfgtypes::Machine =
        serde_dhall::from_file(cfgfile::find(cfgfile::Type::Machine, root, &system.machine))
            .parse()
            .context("Failed to parse machine description")?;

    let processes: Vec<AssignedProcess> = system
        .processes
        .iter()
        .map(|p| internalize_process(root, &machine, p, &system.mappings))
        .collect::<Result<Vec<AssignedProcess>, Error>>()?;

    Ok(RuntimeConfiguration {
        name: system.name.clone(),
        available_memory: machine.available_memory.clone(),
        processes,
    })
}

fn epoxy_verify(root: &Path, system: &cfgtypes::System) -> Result<(), Error> {
    let internalized = configure_system(root, system)?;

    info!("Everything is fine!");
    debug!("Resolved runtime configuration: {:#?}", internalized);

    Ok(())
}

fn epoxy_main() -> Result<(), Error> {
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

    if let Some(_system_matches) = matches.subcommand_matches("verify") {
        epoxy_verify(cfg_root, &system_spec)
    } else {
        Err(format_err!("Unknown subcommand"))
    }
}

fn main() {
    std::process::exit(match epoxy_main() {
        Ok(_) => 0,
        Err(e) => {
            error!("Exiting because of the following chain of errors:");
            for (i, cause) in e.iter_chain().enumerate() {
                error!("Error #{}: {}", i, cause);
            }
            1
        }
    });
}
