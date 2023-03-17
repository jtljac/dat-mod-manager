use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, command, Command, Parser, Subcommand};

use dat_mod_manager::gui_application::gui_application;
use dat_mod_manager::constants;
use dat_mod_manager::mod_info::instance::list_instances;

fn main() -> ExitCode {
    let cli = cmd().get_matches();

    if let Some(subcommand) = cli.subcommand() {
        match subcommand {
            ("list-instances", _) => {
                let instances = list_instances();

                if instances.is_empty() {
                    println!("There are no instances");
                } else {
                    let instance_string: String = instances.iter().map(|(key, instance)| key.to_string() + "\t" + instance.game() + "\n").collect();
                    println!("{instance_string}");
                }
            }
            _ => {
                panic!("Unknown Subcommand")
            }
        }

        ExitCode::SUCCESS
    } else {
        gui_application()
    }
}

fn cmd() -> Command {
    command!()
        .subcommand(
            Command::new("list-instances")
                .about("List all the instances")
        )
        .subcommand(
            Command::new("set-default-instance")
                .about("Set the default instance")
                .long_about("Set the default instance. This is the instance that gets used when the \
                             downloader cannot figure out which instance to download a mod to. \
                             Pass none to be prompted instead")
                .arg(Arg::new("INSTANCE")
                    .allow_hyphen_values(true)
                    .default_value("none")
                )
        )
}
