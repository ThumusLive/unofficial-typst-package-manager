use crate::{
    lexer::CLIOptions,
    utils::{
        check_help, check_smt,
        paths::{
            check_path_dir, check_path_file, create_dir_config, current_config,
            get_current_dir, current_utpm, global_config, global_utpm,
        },
        state::{ErrorState, GoodState},
        Config, ListDependencies, VERSION,
    },
};
use colored::Colorize;
use std::collections::VecDeque;

use super::CommandUTPM;

pub struct New {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for New {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        let globpath = global_utpm();

        print!("◼ Checking if the global utpm folder exist...");

        if !check_path_dir(&globpath) {
            println!("{}", " ✖".red());
            print!("  - Creating the config dir...");
            create_dir_config(&globpath)?;
        }
        println!("{}", " ✔".green());

        print!("◼ Checking if the global utpm config file exist...");

        if !check_path_file(&global_config()) {
            println!("{}", " ✖".red());
            print!("  - Creating the config file...");
            ListDependencies::default().write();
        }
        println!("{}", " ✔".green());

        if check_smt(&self.options, CLIOptions::Global) {
            println!("◼ Skipping local...")
        } else {
            print!("◼ Checking if there is a working directory...");

            let typst_config_dir = current_utpm()?;

            println!("{}", " ✔".green());
            print!("◼ Checking if the current .utpm dir exist...");

            if !check_path_dir(&typst_config_dir) {
                println!("{}", " ✖".red());
                print!("  - Creating the dir...");
                create_dir_config(&typst_config_dir)?;
            }
            println!("{}", " ✔".green());

            let config = get_current_dir()? + "/typst.toml";

            print!("◼ Checking if the current typst.toml file exist...");

            if !check_path_file(&config) {
                println!("{}", " ✖".red());
                print!("  - Creating the file...");
                Config::new(String::from(VERSION), vec![], None).write(&config);
            }
            println!("{}", " ✔".green());
        }

        Ok(GoodState::Good(String::from("All good!")))
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm init");
        println!();
        println!("Description:");
        println!("  This command creates multiple directories. First it creates a directory at ~/.config/utpm");
        println!("  and then it creates a directory at $PWD/.utpm. It add a file called \".config\" in it and");
        println!("  it create a file in the config dir named \".dps\" (for dependencies)");
        println!("  All theses files are written in JSON. Please do not edit them.");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
    }
}
