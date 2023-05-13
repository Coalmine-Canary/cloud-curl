use std::collections::HashMap;

pub struct Profile {
    pub access_key_id: String, // TODO: Safer to use interface?
    pub secret_key: String
}

pub struct Credentials {
    pub profiles: HashMap<String, Profile>
}

impl Credentials {
    pub fn from_string<'s>(string: &'s str) -> Self {
        fn get_map<'s>(block: &'s str) -> HashMap<String, String> {
            let mut map = HashMap::<String, String>::new();
            let mut collect = false;
            let mut key = String::from("");
            let mut value = String::from("");

            for line in block.lines() {
                if line.contains("[") && line.contains("]") {
                    if !&key.is_empty() { // Register previous key value pair if new key detected
                        map.insert(key.clone(), value.clone());
                        key = "".into();
                        value = "".into();
                    }
                    let mut key_started = false;
                    let mut key_completed = false;
                    key = line.trim().chars().filter(|c|
                        {
                            if c == &'[' {
                                key_started = true;
                                return false
                            }
                            if key_started {
                                if c == &']' { key_completed = true }
                                if key_completed {
                                    return false
                                } else {
                                    return true
                                }
                            }
                            false
                        }
                    ).collect();
                }
                value = value + line
            }
            return map;
        }

        fn parse_profile<'s>(block: &'s str) -> Result<Profile, String> {
            let mut access_key_id = String::new();
            let mut secret_key = String::new();

            for line in block.lines() {
                if line.contains('=') {
                    let split: Vec<&str> = line.split('=').filter(|value| {
                        !value.trim().is_empty()
                    }).collect();

                    if split.len() != 2 {
                        return Err("Warning: Possibly malformed credentials file; key value pair is missing. ".into())
                    }

                    if split[0] == "aws_access_key_id" {
                        access_key_id = String::from(split[1]);
                    } else if split[0] == "aws_secret_access_key" {
                        secret_key = String::from(split[1]);
                    }
                }
            }
            if access_key_id.is_empty() && secret_key.is_empty() {
                return Err("Warning: Possibly malformed credentials file; key value pair is missing. ".into())
            }

            return Ok( Profile { access_key_id, secret_key} )
        }

        let str_map = get_map(string);
        let mut profiles = HashMap::<String, Profile>::new();
        for (key, value) in str_map.iter() {
                match parse_profile(&value) {
                    Ok(p) => {
                        profiles.insert(key.clone(), p);
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
        }
        return Credentials { profiles }
    }

}