use std::{collections::HashMap, env, fs};

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct MainArg {
    #[clap(subcommand)]
    pub command: CommandType,

    #[arg(short, long)]
    pub force_write: bool,
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    Init(TypeWrapper),
    Clear,
}

#[derive(Debug, Args)]
pub struct TypeWrapper {
    #[clap(subcommand)]
    pub init_type: Type,
}

#[derive(Subcommand, Debug)]
pub enum Type {
    Sf,
    Salesforce,
    Godot,
}

pub trait Config {
    fn to_config(&self) -> String;

    fn write_config(&self, map: HashMap<String, String>, force: bool) -> Result<String, String>
     {
        let mut current_directory =
            env::current_dir().map_err(|x| format!("Error occurred: {:?}", x))?;
        current_directory.push(".vscode");
        if !current_directory.exists() {
            fs::create_dir(&current_directory).map_err(|x| format!("Error: {:?}", x))?;
        }
        current_directory.push("settings.json");
        if !current_directory.exists() || force {
            let body = map.get(&self.to_config()).ok_or("Nothing found in the config map")?;
            fs::write(&current_directory, body)
                .map_err(|x| format!("Error: {:?}", x))?;
        }

        Ok("Success".to_owned())
    }
}

impl Config for Type {
    fn to_config(&self) -> String {
        match self {
            Type::Sf => String::from("Salesforce"),
            Type::Salesforce => String::from("Salesforce"),
            Type::Godot => String::from("Godot"),
        }
    }
}
