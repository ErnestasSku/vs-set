mod utilities;

use std::{env};

use clap::Parser;
use utilities::{app_args::{CommandType, MainArg, Config}, configs::build_map};

fn main() {
    let args = MainArg::parse();
    let map = build_map();

    match args.command {
        CommandType::Init(config) => {
            let _ = config.init_type.write_config(map, args.force_write)
                .map(|x| println!("{x}"))
                .map_err(|x| println!("{x}"));         
            },
        CommandType::Clear => delete_settings(),
    }
}

fn delete_settings() {
    let current_directory = env::current_dir();
    
    match current_directory {
        Ok(mut path) => {
            path.push(".vscode");
            path.push("settings.json");
            match std::fs::remove_file(path) {
                Ok(_) => println!("Success"),
                Err(err) => println!("Error: {:?}", err),
            }
        }
        Err(err) => println!("Error happened: {:?}", err),
    }

}