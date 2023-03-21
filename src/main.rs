use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, command, Command, Parser, Subcommand};

use dat_mod_manager::gui_application::gui_application;
use dat_mod_manager::constants;
use dat_mod_manager::manager_config::ManagerConfig;
use dat_mod_manager::mod_info::instance::list_instances;

mod util;

fn main() -> ExitCode {
    let cli = cmd().get_matches();

    util::ensure_config_dir();
    ManagerConfig::load_or_create();

    if let Some(subcommand) = cli.subcommand() {
        return match subcommand {
            ("list-instances", _) => list_instances_command(),
            ("set-default-instance", matches) =>
                set_default_instance_command(matches.get_one::<String>("INSTANCE").unwrap()),
            _ => {
                println!("Unknown Subcommand");
                ExitCode::FAILURE
            }
        };


    } else {
        gui_application()
    }
}

fn set_default_instance_command(instance_name: &str) -> ExitCode {
    let instances = list_instances();
    let mut config = ManagerConfig::load_or_create();

    if instance_name != "none" && !instances.contains_key(instance_name) {
        println!("Unknown instance: {instance_name}");
        return ExitCode::FAILURE
    }

    config.default_instance = if instance_name == "none" {""} else {instance_name}.to_string();
    if let Err(err) = config.save() {
        println!("Failed to save new default instance:\n{err}");
        return ExitCode::FAILURE
    }

    if instance_name == "none" {
        println!("Successfully cleared default instance")
    } else {
        println!("Successfully set default instance to {instance_name}");
    }

    ExitCode::SUCCESS
}

fn list_instances_command() -> ExitCode {
    let instances = list_instances();

    if instances.is_empty() {
        println!("There are no instances");
    } else {
        let instance_string: String = instances.iter()
            .map(|(key,     instance)| {
                let config = ManagerConfig::load_or_create();
                let selected = if key == &config.default_instance {
                    "*"
                } else {
                    ""
                };
                format!("{key}{selected}\t{}\n", instance.game())
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{instance_string}");
    }

    ExitCode::SUCCESS
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
                             Pass none to be prompted everytime instead")
                .arg(
                    Arg::new("INSTANCE")
                    .allow_hyphen_values(true)
                        .required(true)
                )
        )
}
