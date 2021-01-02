#[macro_use]
extern crate failure;

use clap::{App, Arg, SubCommand};
use failure::Error;
use failure::ResultExt;
use log::{debug, error, info};
use serde_dhall;
use std::path::Path;

mod cfgfile;
mod cfgtypes;

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
        .subcommand(SubCommand::with_name("verify")
                    .about("Verify the system configuration."))
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

    Ok(())
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
