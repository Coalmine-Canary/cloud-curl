use once_cell::sync::Lazy;
use serde::Deserializer;
use toml::Value;
use toml::{Table, map::Map};


use std::fs::File;
use std::io::Read;
use std::io::read_to_string;

use std::path::Path;
//use std::thread::__FastLocalKeyInner;

use crate::config;
use crate::aws::credentials;

pub fn get_credentials() -> Result<(String, String), String> {
    // Checks environment variables and aws credentials file

    let env: &Lazy<config::Env> = &config::ENV;

    if env.ACCESS_KEY.is_some() && env.SECRET_KEY.is_some() {
        return Ok((env.ACCESS_KEY.clone().unwrap(), env.SECRET_KEY.clone().unwrap()))
    }

    let profile = match &env.AWS_PROFILE {
        Some(p) => p.clone(),
        None => "default".into()
    };

    let credentials = credentials::Credentials::get(&profile)?;

    return Ok((credentials.access_key_id.clone(), credentials.secret_key.clone()))

}


// AWS_SHARED_CREDENTIALS_FILE

// fn get_credentials() -> Result<(String, String), String> {
//     // Checks environment variables and aws credentials file

//     let env_reference = &mut config::ENV;
//     let mut env = Lazy::get_mut(env_reference).unwrap();
//     let mut access_key = String::from(env.ACCESS_KEY.as_ref().unwrap_or(&"".into()));
//     let mut secret_key = String::from(env.SECRET_KEY.as_ref().unwrap_or(&"".into()));;
//     let profile = env.AWS_PROFILE.as_ref().unwrap_or(&"default".into()).clone();

//     if access_key == "" || secret_key == "" {
//         let home = match &env.HOME {
//             Some(h) => h,
//             None => { return Err("Could not find home directory. ".into()) }
//         };
//         let toml = match load_toml(format!("{}/{}", home, ".aws/credentials")) {
//             Ok(o) => match o {
//                 Some(t) => t,
//                 None => Map::new()
//             },
//             Err(e) => return Err(e)
//         };
//         match toml.get(&profile) {
//             Some(s) => {
//                 match s {
//                     Value::Table(t) => {
//                         if t.get("aws_access_key_id").is_some() { access_key = t.get("aws_access_key_id").unwrap().to_string(); };
//                         if t.get("aws_secret_access_key").is_some() { secret_key = t.get("aws_secret_access_key").unwrap().to_string(); };
//                     },
//                     _ => {
//                         eprintln!("Warning: Invalid credentials file. ");
//                     }
//                 }
//             },
//             None => {}
//         };
//     };
//     Ok((access_key, secret_key))
// }