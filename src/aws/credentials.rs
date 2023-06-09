use std::collections::HashMap;
use std::path::PathBuf;

use crate::aws::config;
use crate::config::ENV;

pub struct Credentials {
    pub access_key_id: String, // TODO: Safer to use interface?
    pub secret_key: String
}

impl Credentials {

    pub fn get<'s>(profile: &'s str) -> Result<Self, String> { // May need to add profile: String
        // Return credential struct for given profile
        let home = match &ENV.HOME {
            Some(h) => h,
            None => { return Err("Could not find home directory. ".into()) }
        };
        let credential_vars = config::parse(PathBuf::from(format!("{}/{}", home, ".aws/credentials")), profile)?;
        let access_key_id = match credential_vars.get("aws_access_key_id") {
            Some(a) => String::from(a),
            None => return Err("Could not retrieve AWS access key with given profile. ".into())
        };
        let secret_key  = match credential_vars.get("aws_secret_access_key") {
            Some(s) => String::from(s),
            None => return Err("Could not retrieve AWS secret key with given profile. ".into())
        };

        return Ok( Credentials { access_key_id, secret_key } )
    }

}
pub struct Profile {
    region: String
}

impl Profile {
    pub fn get<'s>(profile: &'s str) -> Result<Self, String> { // May need to add profile: String
        // Return credential struct for given profile
        let home = match &ENV.HOME {
            Some(h) => h,
            None => { return Err("Could not find home directory. ".into()) }
        };
        let profile_vars = config::parse(PathBuf::from(format!("{}/{}", home, ".aws/config")), profile)?;

        return Ok(
            Profile {
                region: profile_vars.get("region").unwrap_or(&String::from("us-east-1")).into()
            }
        )
    }

}

