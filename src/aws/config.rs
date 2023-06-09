use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn parse<'s>(path: PathBuf, profile: &'s str) -> Result<HashMap<String, String>, String> {
    // Parse file in .aws/{config,credentials} format
    let block: String = match read_to_string(path) {
        Ok(b) => b,
        Err(e) => return Err(format!("{}", e))
    };

    return Ok(get_profile(&block, profile)?)
}

fn get_profile<'s>(block: &'s str, profile: &'s str) -> Result<HashMap<String, String>, String> {
    // In profile/credential file, return fields
    let profile_blocks = get_map(block);
    let block = match profile_blocks.get(profile as &str) {
        Some(b) => b,
        None => return Err("Profile could not be found. ".into()) // TODO: Review, more detail
    };

    let mut config_pairs = HashMap::<String, String>::new();

    for line in block.lines() {
        if line.contains('=') {
            let split: Vec<&str> = line.split('=').filter(|value| {
                !value.trim().is_empty()
            }).map(|value| value.trim()).collect();

            if split.len() != 2 {
                return Err("Error: Possibly malformed credentials file; key value pair is missing. ".into())
            }
            config_pairs.insert(split[0].into(), split[1].into());
        }
    }

    return Ok(config_pairs)
}

fn get_map<'s>(block: &'s str) -> HashMap<String, String> {
    // Returns profile_name -> block
    let mut map = HashMap::<String, String>::new();
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
        } else {
            value = value + "\n" + line;
        }
    }

    if !&key.is_empty() { // Register previous key value pair if new key detected
        map.insert(key.clone(), value.clone());
    }

    return map;
}

