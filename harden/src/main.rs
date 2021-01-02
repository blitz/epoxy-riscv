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

    // TODO Maybe it's better to directly refer to the resources here instead of having a
    // redirection via the resource name.
    resources: Vec<cfgtypes::Mapping>,
}

#[derive(Debug)]
struct InternalizedSystem {
    name: String,
    machine: cfgtypes::Machine,
    processes: Vec<InternalizedProcess>,
}

/// Convert the global list of resource mappings to a per-process list of mappings.
///
/// This removes all mappings that do not refer to the process. It also removes the process name
/// prefixes in the `to` member.
fn to_process_mappings(
    process_name: &str,
    mappings: &[cfgtypes::Mapping],
) -> Result<Vec<cfgtypes::Mapping>, Error> {
    let split_mappings = mappings
        .iter()
        .map(|m| -> Result<(String, String, String), Error> {
            // TODO There is `split_once` in Nightly.
            let dot_pos =
                m.to.find('.')
                    .ok_or_else(|| format_err!("Missing '.' in mapping destination: {}", m.to))?;
            let before = &m.to[..dot_pos];
            let after = &m.to[dot_pos + 1..];

            Ok((m.from.clone(), before.to_string(), after.to_string()))
        })
        .collect::<Result<Vec<(String, String, String)>, Error>>()?;

    Ok(split_mappings
        .into_iter()
        .filter(|(_, to_proc, _)| to_proc == process_name)
        .map(|(from, _, to_input)| cfgtypes::Mapping { to: to_input, from })
        .collect())
}

fn internalize_process(
    root: &Path,
    process: &cfgtypes::Process,
    mappings: &[cfgtypes::Mapping],
) -> Result<InternalizedProcess, Error> {
    let app_cfg_file = cfgfile::find(cfgfile::Type::Application, root, &process.program);
    info!(
        "Using {} as configuration file for process {}",
        app_cfg_file.display(),
        process.name
    );

    let program: cfgtypes::Application = serde_dhall::from_file(app_cfg_file)
        .parse()
        .context("Failed to parse machine description")?;

    Ok(InternalizedProcess {
        name: process.name.clone(),
        resources: to_process_mappings(&process.name, mappings)
            .context("Failed to read resource mappings")?,
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
        .map(|p| internalize_process(root, p, &system.mappings))
        .collect::<Result<Vec<InternalizedProcess>, Error>>()?;

    Ok(InternalizedSystem {
        name: system.name.clone(),
        machine,
        processes,
    })
}

fn epoxy_verify(root: &Path, system: &cfgtypes::System) -> Result<(), Error> {
    let internalized = internalize_system(root, system)?;

    debug!("Internalized system description: {:#?}", internalized);

    // TODO Check that process names are unique.

    // TODO Check that resource names are unique.

    // TODO Check that `needs` names are unique.

    // TODO Check if every `needs` element is satisfied.

    // TODO Check if every `to` actually matches a need. (This is not the same as the check above,
    // because we could have extra mappings to non-existent needs).

    // TODO Check that resources are not used multiple times.

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
