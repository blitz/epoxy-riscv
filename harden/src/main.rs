#[macro_use]
extern crate failure;

use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use failure::ResultExt;
use log::{debug, error, info};
use serde_dhall;
use std::path::Path;

mod cfgfile;
mod cfgtypes;

#[derive(Debug)]
struct InternalizedProcess {
    name: String,
    program: cfgtypes::Application,
}

#[derive(Debug)]
struct InternalizedSystem {
    name: String,
    machine: cfgtypes::Machine,
    processes: Vec<InternalizedProcess>,
}

fn internalize_process(
    root: &Path,
    process: &cfgtypes::Process,
) -> Result<InternalizedProcess, Error> {
    let app_cfg_file = cfgfile::find(cfgfile::Type::Application, root, &process.program);
    debug!(
        "Using {} as configuration file for process {}",
        app_cfg_file.display(),
        process.name
    );

    let program: cfgtypes::Application = serde_dhall::from_file(app_cfg_file)
        .parse()
        .context("Failed to parse machine description")?;

    Ok(InternalizedProcess {
        name: process.name.clone(),
        program,
    })
}

/// Take a system description as it comes in from the config files and read all other configurations
/// it refernces.
fn internalize_system(root: &Path, system: &cfgtypes::System) -> Result<InternalizedSystem, Error> {
    let machine: cfgtypes::Machine =
        serde_dhall::from_file(cfgfile::find(cfgfile::Type::Machine, root, &system.machine))
            .parse()
            .context("Failed to parse machine description")?;

    let processes: Vec<InternalizedProcess> = system
        .processes
        .iter()
        .map(|p| internalize_process(root, p))
        .collect::<Result<Vec<InternalizedProcess>, Error>>()?;

    Ok(InternalizedSystem {
        name: system.name.clone(),
        machine,
        processes,
    })
}

fn epoxy_verify(root: &Path, system: &cfgtypes::System) -> Result<(), Error> {
    let internalized = internalize_system(root, system)?;

    debug!("Internalized system description: {:?}", internalized);

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

    debug!("System description is: {:?}", system_spec);

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
