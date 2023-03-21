use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, command, Command, Parser, Subcommand, value_parser};

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
            ("create-instance", matches) => {
                create_instance_command(matches.get_one::<String>("NAME"),
                                        matches.get_one::<PathBuf>("BASE_PATH"),
                                        matches.get_one::<PathBuf>("MODS_PATH"),
                                        matches.get_one::<PathBuf>("DOWNLOADS_PATH"),
                                        matches.get_one::<PathBuf>("OVERWRITE_PATH"),
                                        matches.get_one::<PathBuf>("PROFILES_PATH"),
                                        matches.get_one::<bool>("DEFAULT"))
            }
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
        .subcommand(
            Command::new("create-instance")
                .about("Create a new instance")
                .after_help("For the paths (besides the BASE_PATH), relative paths will be relative to the BASE_PATH, absolute paths will work as expected\n\
                Any arguments or options not provided will be prompted for")
                .arg(
                    Arg::new("NAME")
                        .allow_hyphen_values(true)
                        .help("The name of the new profile")
                )
                .arg(
                    Arg::new("BASE_PATH")
                        .long("base-path")
                        .short('b')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The base directory of the instance")
                )
                .arg(
                    Arg::new("MODS_PATH")
                        .long("mods-path")
                        .short('m')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which mods are stored")
                )
                .arg(
                    Arg::new("DOWNLOADS_PATH")
                        .long("downloads-path")
                        .short('d')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which mods are downloaded to")
                )
                .arg(
                    Arg::new("OVERWRITE_PATH")
                        .long("overwrite-path")
                        .short('o')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which runtime changes are written to")
                )
                .arg(
                    Arg::new("PROFILE_PATH")
                        .long("profile-path")
                        .short('p')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which profile information is stored")
                )
                .arg(
                    Arg::new("DEFAULT")
                        .long("default")
                        .short('D')
                        .action(ArgAction::SetTrue)
                        .help("Set the newly created profile as the default profile")
                )
        )
}
