use std::collections::HashMap;

struct Profile {
    access_key_id: String,
    secreet_key: String
}

struct Credentials {
    profiles: HashMap<String, Profile>
}

impl Credentials {
    fn from_string<'s>(string: &'s str) {
        fn get_map<'s>(block: &'s str) -> HashMap<String, String> {
            let mut map = HashMap::<String, String>::new();
            let mut collect = false;
            let mut key = String::from("");
            let mut value = String::from("");

            for line in block.lines() {
                if line.contains("[") && line.contains("]") {
                    if collect {
                        collect = false;
                        map.insert(key, value);
                        key = "".into();
                        value = "".into();
                    } else {
                        collect = true
                    };

                    let mut started = false;
                    let mut completed = false;
                    key = line.trim().chars().filter(|c|
                        {
                            if c == &']' { completed = true; return false }
                            else if completed { return false }
                            else if c == &'[' {
                                started = true;
                                return false
                            }
                            true
                        }
                    ).collect();
                    begin = true
                }
            }
            return map;
        }
    }

}