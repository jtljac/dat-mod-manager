use std::collections::HashMap;
use std::{fs, io};
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Arg, ArgAction, ArgGroup, command, Command, Parser, Subcommand, value_parser};
use clap::builder::PossibleValuesParser;
use clap::ValueHint::DirPath;
use gtk::glib::PropertyGet;
use lazy_static::lazy_static;
use regex::Regex;

use dat_mod_manager::gui_application::gui_application;
use dat_mod_manager::constants;
use dat_mod_manager::manager_config::ManagerConfig;
use dat_mod_manager::mod_info::instance;
use dat_mod_manager::mod_info::instance::{Instance, get_instances, list_instances};

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

                create_instance_command(matches.get_one::<String>("NAME").cloned().unwrap(),
                                        matches.get_one::<String>("GAME").cloned().unwrap(),
                                        matches.get_one::<PathBuf>("BASE_PATH"),
                                        matches.get_one::<PathBuf>("MODS_PATH"),
                                        matches.get_one::<PathBuf>("DOWNLOADS_PATH"),
                                        matches.get_one::<PathBuf>("OVERWRITE_PATH"),
                                        matches.get_one::<PathBuf>("PROFILES_PATH"),
                                        matches.get_one::<bool>("DEFAULT").unwrap().clone())
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

fn is_name_valid(name: &str, instances: HashMap<String, Instance>, feedback: bool) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\w \-.]+").unwrap();
    }
    let instances = get_instances();

    if name.is_empty() {
        if feedback {println!("You must provide a profile name")}
        false
    } else if instances.contains_key(name) {
        if feedback {println!("A profile with that name already exists")};
        false
    } else if !RE.is_match(&name) {
            if feedback {println!("That name does not meet the requirements (Letters, numbers, -, _, .)")};
        false
    } else {
        true
    }
}

fn create_instance_command(name: String, game: String, base_path: Option<&PathBuf>, mods_path: Option<&PathBuf>, downloads_path: Option<&PathBuf>, overwrite_path: Option<&PathBuf>, profiles_path: Option<&PathBuf>, default: bool) -> ExitCode {
    let instances = get_instances();
    let mut config = ManagerConfig::load_or_create();
    // Get name

    // Get base_path
    let base_path = match base_path {
        None => {
            println!("Enter the base directory path (leave empty for {}):", constants::instance_data_dir().join(&name).display());
            let mut string_path: String = "".to_string();
            io::stdin()
                .read_line(&mut string_path)
                .expect("Failed to get user input");
            if string_path.trim().is_empty() {
                constants::instance_data_dir().join(&name)
            } else {
                PathBuf::from(string_path.trim())
            }
        }
        Some(path) => path.clone()
    };

    // Get mods path
    let mods_path = match mods_path {
        None => {
            println!("Enter the mods directory path, relative paths will be relative to the base directory (leave empty for ./mods):");
            let mut string_path: String = "".to_string();
            io::stdin()
                .read_line(&mut string_path)
                .expect("Failed to get user input");
            if string_path.trim().is_empty() {
                PathBuf::from("./mods")
            } else {
                PathBuf::from(string_path.trim())
            }
        }
        Some(path) => path.clone()
    };

    // Get downloads path
    let downloads_path = match downloads_path {
        None => {
            println!("Enter the downloads directory path, relative paths will be relative to the base directory (leave empty for ./downloads):");
            let mut string_path: String = "".to_string();
            io::stdin()
                .read_line(&mut string_path)
                .expect("Failed to get user input");
            if string_path.trim().is_empty() {
                PathBuf::from("./downloads")
            } else {
                PathBuf::from(string_path.trim())
            }
        }
        Some(path) => path.clone()
    };

    // Get overwrite path
    let overwrite_path = match overwrite_path {
        None => {
            println!("Enter the overwrite directory path, relative paths will be relative to the base directory (leave empty for ./overwrite):");
            let mut string_path: String = "".to_string();
            io::stdin()
                .read_line(&mut string_path)
                .expect("Failed to get user input");
            if string_path.trim().is_empty() {
                PathBuf::from("./overwrite")
            } else {
                PathBuf::from(string_path.trim())
            }
        }
        Some(path) => path.clone()
    };

    // Get Profiles path
    let profiles_path = match profiles_path {
        None => {
            println!("Enter the profiles directory path, relative paths will be relative to the base directory (leave empty for ./profiles):");
            let mut string_path: String = "".to_string();
            io::stdin()
                .read_line(&mut string_path)
                .expect("Failed to get user input");
            if string_path.trim().is_empty() {
                PathBuf::from("./profiles")
            } else {
                PathBuf::from(string_path.trim())
            }
        }
        Some(path) => path.clone()
    };

    // Create instance
    match instance::create_instance(&name, &game, &base_path, &mods_path, &downloads_path, &overwrite_path, &profiles_path) {
        Ok(_) => {
            println!("Successfully created instance")
        }
        Err(err) => {
            println!("Failed to create instance, error given is:\n{}", err.to_string());
            return ExitCode::FAILURE
        }
    };

    config.default_instance = name;
    if let Err(err) = config.save() {
        println!("Failed to save default instance:\n{err}");
        return ExitCode::FAILURE
    }

    ExitCode::SUCCESS
}

fn set_default_instance_command(instance_name: &str) -> ExitCode {
    let instances = get_instances();
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
    let instances = get_instances();

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
                .group(
                    ArgGroup::new("Paths")
                        .args(&["BASE_PATH", "MODS_PATH", "DOWNLOADS_PATH", "OVERWRITE_PATH", "PROFILES_PATH"])
                )

                .arg(
                    Arg::new("NAME")
                        .allow_hyphen_values(true)
                        .help("The name of the new profile")
                        .required(true)
                )
                .arg(
                    Arg::new("GAME")
                        .allow_hyphen_values(true)
                        .help("The game this profile is for")
                        .required(true)
                        // .value_parser() TODO: available games
                )
                .arg(
                    Arg::new("BASE_PATH")
                        .long("base-path")
                        .short('b')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The base directory of the instance")
                        .value_hint(DirPath)
                )
                .arg(
                    Arg::new("MODS_PATH")
                        .long("mods-path")
                        .short('m')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which mods are stored")
                        .value_hint(DirPath)
                )
                .arg(
                    Arg::new("DOWNLOADS_PATH")
                        .long("downloads-path")
                        .short('d')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which mods are downloaded to")
                        .value_hint(DirPath)
                )
                .arg(
                    Arg::new("OVERWRITE_PATH")
                        .long("overwrite-path")
                        .short('o')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which runtime changes are written to")
                        .value_hint(DirPath)
                )
                .arg(
                    Arg::new("PROFILES_PATH")
                        .long("profiles-path")
                        .short('p')
                        .value_parser(value_parser!(std::path::PathBuf))
                        .help("The directory in which profiles information is stored")
                        .value_hint(DirPath)
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
