use std::collections::HashMap;
use std::{io, thread};
use std::fmt::Write;
use std::io::stdin;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;
use clap::{Arg, ArgAction, ArgGroup, command, Command, value_parser, ValueHint};
use clap::builder::ValueParser;
use clap::ValueHint::DirPath;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use regex::Regex;
use url::Url;

use dat_mod_manager::gui_application::gui_application;
use dat_mod_manager::constants;
use dat_mod_manager::manager_config::ManagerConfig;
use dat_mod_manager::mod_info::instance;
use dat_mod_manager::mod_info::instance::{Instance, get_instances, delete_instance, list_instances};
use dat_mod_manager::plugin::downloader::DownloadFailed;
use dat_mod_manager::plugin::plugin_manager::PluginManager;
use dat_mod_manager::plugin::PluginManager;

use dat_mod_manager::util::delete_dir_with_callback;

mod util;

fn main() -> ExitCode {
    util::ensure_config_dir();
    let mut config = ManagerConfig::load_or_create();

    let cli = cmd(&config).get_matches();
    let mut plugin_man = PluginManager::default();
    plugin_man.register_plugins();

    if let Some(subcommand) = cli.subcommand() {
        return match subcommand {
            ("list-instances", _) => list_instances_command(&config),
            ("set-default-instance", matches) =>
                set_default_instance_command(&mut config, matches.get_one::<String>("INSTANCE").unwrap()),
            ("create-instance", matches) =>
                create_instance_command(&mut config,
                                        matches.get_one::<String>("NAME").cloned().unwrap(),
                                        matches.get_one::<String>("GAME").cloned().unwrap(),
                                        matches.get_one::<PathBuf>("BASE_PATH"),
                                        matches.get_one::<PathBuf>("MODS_PATH"),
                                        matches.get_one::<PathBuf>("DOWNLOADS_PATH"),
                                        matches.get_one::<PathBuf>("OVERWRITE_PATH"),
                                        matches.get_one::<PathBuf>("PROFILES_PATH"),
                                        *matches.get_one::<bool>("DEFAULT").unwrap()),
            ("delete-instance", matches) =>
                delete_instance_command(&mut config,
                                        matches.get_one::<String>("NAME").cloned().unwrap(),
                                        *matches.get_one::<bool>("CLEAR").unwrap(),
                                        *matches.get_one::<bool>("FORCE").unwrap()),
            ("download", matches) => {

                download_command(&mut config, &mut plugin_man,
                                 matches.get_one::<Url>("URL").cloned().unwrap(),
                                 matches.get_one::<String>("INSTANCE").cloned(),
                                 *matches.get_one::<bool>("NO-PROMPT").unwrap())
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

fn list_instances_command(config: &ManagerConfig) -> ExitCode {
    let instances = get_instances();

    if instances.is_empty() {
        println!("There are no instances");
    } else {
        let instance_string: String = instances.iter()
            .map(|(key,     instance)| {
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

fn set_default_instance_command(config: &mut ManagerConfig, instance_name: &str) -> ExitCode {
    let instances = get_instances();

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

fn create_instance_command(
    config: &mut ManagerConfig,
    name: String,
    game: String,
    base_path: Option<&PathBuf>,
    mods_path: Option<&PathBuf>,
    downloads_path: Option<&PathBuf>,
    overwrite_path: Option<&PathBuf>,
    profiles_path: Option<&PathBuf>,
    default: bool
) -> ExitCode {
    let instances = get_instances();
    let mut config = ManagerConfig::load_or_create();

    // Check name
    if name.is_empty() {
        println!("You must provide a profile name");
        return ExitCode::FAILURE
    } else if instances.contains_key(&name) {
        println!("A profile with that name already exists");
        return ExitCode::FAILURE
    } else if !Regex::new(r"[\w \-.]+").unwrap().is_match(&name) {
        println!("That name does not meet the requirements (Letters, numbers, -, _, .)");
        return ExitCode::FAILURE
    }

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

    match instance::create_instance(&name, &game, &base_path, &mods_path, &downloads_path, &overwrite_path, &profiles_path) {
        Ok(_) => {
            println!("Successfully created instance")
        }
        Err(err) => {
            println!("Failed to create instance, error given is:\n{}", err);
            return ExitCode::FAILURE
        }
    };

    if default {
        config.default_instance = name;
        if let Err(err) = config.save() {
            println!("Failed to save default instance:\n{err}");
            return ExitCode::FAILURE
        }
    }

    ExitCode::SUCCESS
}

fn delete_instance_command(config: &mut ManagerConfig, name: String, remove: bool, force: bool) -> ExitCode {
    let instance = match Instance::from_name(&name) {
        Ok(instance) => instance,
        Err(_) => {
            println!("No instance with that name");
            return ExitCode::FAILURE
        }
    };

    if !force {
        println!("Are you sure you want to delete instance: {name}? (y/n)");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to get user input");

        if !answer.starts_with('y') {
            println!("Instance not deleted");
            return ExitCode::FAILURE
        }
    }

    if remove {
        println!("Removing instance data");
        let m = MultiProgress::new();
        let style = ProgressStyle::with_template("{spinner:.green} {prefix} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {msg:!40} {human_pos}/{human_len} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-");

        let callback = |pb: &ProgressBar, total: u32, progress:u32, file: &str| {
            pb.set_length(total as u64);
            pb.set_position(progress as u64);
            pb.set_message(file.to_string());
        };

        let deletions = HashMap::from([
            ("Downloads", instance.downloads_path()),
            ("Mods", instance.mods_path()),
            ("Overwrite", instance.overwrite_path()),
            ("Profiles", instance.profiles_path()),
        ]);

        let handles: Vec<_> = deletions.iter().map(|(key, value)| {
            let pb = m.add(ProgressBar::new(100));
            pb.set_style(style.clone());
            pb.set_prefix(key.to_string());
            let value = value.clone();
            thread::spawn(move || {
                delete_dir_with_callback(value, |total: u32, progress:u32, file: &str| {
                    callback(&pb, total, progress, file);
                });

                pb.finish_with_message("waiting...")
            })
        }).collect();

        for handle in handles {
            let _ = handle.join();
        }

        let pb = m.add(ProgressBar::new(100));
        pb.set_style(style);
        pb.set_prefix("Instance");

        delete_dir_with_callback(instance.base_path().to_path_buf(), |total: u32, progress:u32, file: &str| {
            callback(&pb, total, progress, file);
        });

        pb.finish_with_message("done")
    }

    match delete_instance(&name) {
        Ok(_) => {
            println!("Successfully deleted instance");
        }
        Err(err) => {
            println!("Failed to delete instance file, given error was: \n{err}")
        }
    }

    if config.default_instance == name {
        config.default_instance = "".to_string();
        if let Err(err) = config.save() {
            println!("Failed to clear default instance:\n{err}");
            return ExitCode::FAILURE
        }
    }

    ExitCode::SUCCESS
}

fn download_command(config: &mut ManagerConfig, plugin_man: &mut PluginManager, url: Url, instance: Option<String>, no_prompt: bool) -> ExitCode {
    // Get instance
    let instance: Instance = {
        let instances = list_instances();
        let name: String = match instance {
            None => {
                if !config.default_instance.is_empty() {
                    config.default_instance.clone()
                } else if no_prompt {
                    println!("No default instance configured");
                    return ExitCode::FAILURE
                } else if false { // && atty::is(atty::Stream::Stdin) {
                    let mut name = String::new();
                    stdin().read_line(&mut name)
                        .expect("Failed to get user input");
                    if instances.contains(&name.trim().to_string()) {
                        name
                    } else {
                        println!("Unknown instance: {name}");
                        return ExitCode::FAILURE
                    }
                } else {
                    notify_rust::Notification::new()
                        .appname("Dat Mod Manager")
                        .icon("dat-mod-manager")
                        .summary("No instance specified or default instance configured");
                    return ExitCode::FAILURE
                }
            }
            Some(name) => name
        };

        match Instance::from_name(&name) {
            Ok(instance) => instance,
            Err(err) => {
                println!("Failed to get instance, error: {err}");
                return ExitCode::FAILURE
            }
        }
    };

    // Select downloader
    let protocol = match url.scheme() {
        "" => "http",
        scheme => scheme
    };
    let downloaders = plugin_man.downloaders().get_downloaders_for_protocol(protocol);

    if (downloaders.is_empty()) {
        println!("Failed to find downloader for that protocol, is it supposed to be http?");
        return ExitCode::FAILURE
    }

    let downloader = downloaders[0];

    let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {msg:!40} {bytes}/{total_bytes} [{binary_bytes_per_sec}] ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");
    let pb = ProgressBar::new(100000);
    pb.set_style(style);

    let mut callback = |total: u64, progress: u64, file: String| {
        // pb.set_length(total);
        pb.set_position(progress);
        pb.set_message(file);
    };

    // Download
    let result = downloader.download(&url, instance.downloads_path(), &mut callback);

    pb.finish_and_clear();

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn cmd(config: &ManagerConfig) -> Command {
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
                        .args(["BASE_PATH", "MODS_PATH", "DOWNLOADS_PATH", "OVERWRITE_PATH", "PROFILES_PATH"])
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
                        .help("The directory in which profiles information is stored")
                        .value_parser(value_parser!(std::path::PathBuf))
                        .value_hint(DirPath)
                )
                .arg(
                    Arg::new("DEFAULT")
                        .long("default")
                        .short('D')
                        .help("Set the newly created profile as the default profile")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("delete-instance")
                .about("Delete an instance, optionally removing all it's content")
                .arg(
                    Arg::new("NAME")
                        .allow_hyphen_values(true)
                        .help("The name of the profile to delete")
                        .required(true)
                )
                .arg(
                    Arg::new("CLEAR")
                        .long("clear")
                        .short('c')
                        .help("Remove this instances data (mods, downloads, etc)")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("FORCE")
                        .long("force")
                        .short('f')
                        .help("Skip the prompt double-checking you're sure you want to delete the profile")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("download")
                .about("Download a mod")
                .long_about("Download a mod using the given URL, uses the default instance if no instance is specified")
                .arg(
                    Arg::new("URL")
                        .value_hint(ValueHint::Url)
                        .help("The URL of the mod to download")
                        .required(true)
                        .value_parser(value_parser!(Url))
                )
                .arg(
                    Arg::new("INSTANCE")
                        .long("instance")
                        .short('i')
                        .help("Specify an instance to download the mod to")
                )
                .arg(
                    Arg::new("NO-PROMPT")
                        .long("no-prompt")
                        .short('n')
                        .help("Fail instead of prompting if no instance is set")
                        .action(ArgAction::SetTrue)
                )
        )
}
