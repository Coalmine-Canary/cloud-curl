use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use clap::{Subcommand, Args};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use serde_yaml;

const HOME_CONFIG_DIR: &str = ".config/cloud-curl";
const SYSTEM_CONFIG_DIR: &str = "/etc/cloud-curl/";
const CONFIG_FILE_NAME: &str = "config.yml";

#[derive(Debug)]
pub struct Env {
    pub AWS_PROFILE: Option<String>,
    pub ACCESS_KEY: Option<String>,
    pub SECRET_KEY: Option<String>,
    pub HOME: Option<String>
}
pub fn get_env() -> Env {
    let mut vars: HashMap<String, String> = env::vars().collect();
    let mut profile: Option<String> = None;
    let mut access_key: Option<String> = None;
    let mut secret_key: Option<String> = None;
    let mut home: Option<String> = None;

    vars.drain().for_each(|(key, val)|
        match key.as_str() {
            "AWS_PROFILE" => { profile.replace(val); }
            "AWS_ACCESS_KEY_ID"  => { access_key.replace(val); }
            "AWS_SECRET_ACCESS_KEY"  => { secret_key.replace(val); }
            "HOME"        => { home.replace(val); }
            _ => {}
        }
    );

    Env {
        AWS_PROFILE: profile,
        ACCESS_KEY: access_key,
        SECRET_KEY: secret_key,
        HOME: home
    }
}
pub static ENV: Lazy<Env> = Lazy::new(||get_env());

fn get_and_create_path() -> Result<PathBuf, String> {
    let mut path = PathBuf::new();
    path.push(match env::vars().find( |var| var.0 == "HOME" ) {
        Some(v) => format!("{}/{}", v.1, HOME_CONFIG_DIR),
        None => SYSTEM_CONFIG_DIR.into()
    });

    match create_dir_all(&path) {
        Ok(_) => {},
        Err(e) => { return Err(e.to_string())}
    };

    path.push(CONFIG_FILE_NAME);

    Ok(path)
}

#[derive(Args)]
pub struct SettingsFlags {
    #[arg(long)]
    endpoint: bool,
    #[arg(long)]
    region: bool
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
#[derive(Args)]
pub struct Settings {
    #[arg(long)]
    endpoint: Option<String>,
    #[arg(long)]
    region: Option<String>
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "endpoint: {}\n", self.endpoint.as_ref().unwrap_or(&String::new()))?;
        write!(f, "region: {}", self.region.as_ref().unwrap_or(&String::new()))
    }
}

impl Settings {
    fn read() -> Result<Self, String> {
        let path = match get_and_create_path() {
            Ok(p) => p,
            Err(e) => return Err(e)
        };

        let mut file = match OpenOptions::new()
            .create(true).append(true).read(true)
            .open(path)
            {
                Ok(f) => f,
                Err(e) => { return Err(e.to_string()) }
            };

        let mut yaml = String::new();
        match file.read_to_string(&mut yaml) {
            Ok(y) => y,
            Err(e) => { return Err(e.to_string()) }
        };

        match serde_yaml::from_str::<Self>(&yaml) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string())
        }
    }

    fn output_fields(&self, flags: SettingsFlags) {
        if flags.endpoint {
            println!("endpoint: {}", self.endpoint.as_ref().unwrap_or(&String::new()));
        }

        if flags.region {
            println!("region: {}", self.region.as_ref().unwrap_or(&String::new()))
        }

        if ! ( flags.region || flags.endpoint ) {
            println!("{}", self)
        }
    }

    fn merge(&mut self, new_settings: Settings) {
        // New settings fields overwrite original struct fields
        if new_settings.endpoint.is_some() { self.endpoint = new_settings.endpoint }
        if new_settings.region.is_some() { self.region = new_settings.region }
    }

    fn write(&self) -> Result<(), String> {
        let yaml = match serde_yaml::to_string(&self) {
            Ok(y) => y,
            Err(e) => return Err(e.to_string())
        };

        let path = match get_and_create_path() {
            Ok(p) => p,
            Err(e) => return Err(e)
        };

        let mut file = match OpenOptions::new()
            .create(true).write(true).truncate(true)
            .open(path) {
                Ok(f) => f,
                Err(e) => { return Err(e.to_string()) }
            };

        match file.write(yaml.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string())
        }

    }
}

#[derive(Subcommand)]
/// Alter configuration settings
pub enum Commands {
    // Settings' subcommands
    #[command(name = "get")]
    Get(SettingsFlags),
    #[command(name = "set")]
    Set(Settings)
}

pub async fn handle(commands: Commands) -> Result<(), String> {
    let mut settings = match Settings::read() {
        Ok(s) => s,
        Err(e) => { return Err(e) }
    };
    match commands {
        Commands::Get(flags) => {
            settings.output_fields(flags)
        },
        Commands::Set(args) => {
            settings.merge(args);
            return settings.write()
        }
    };
    Ok(())
}
