use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, command, Command, Parser, Subcommand};

use dat_mod_manager::gui_application::gui_application;
use dat_mod_manager::constants;

fn main() -> ExitCode {
    let cli = cmd().get_matches();

    match cli.subcommand() {
        Some(("list-instances", sub_matches)) => {
            let instance_path = constants::config_dir().join("/instances");
            if !instance_path.exists() {
                println!("No instances")
            }


        }
        _ => {}
    }

    if cli.get_flag("cli") {
        ExitCode::SUCCESS
    } else {
        gui_application()
    }
}

fn cmd() -> Command {
    command!()
        .arg(Arg::new("cli")
            .long("cli")
            .action(ArgAction::SetTrue))
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
