use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

use crate::network::get_local_ipaddr;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    local_ipaddr: String,
    hostname_ip_map: HashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Config {
            local_ipaddr: get_local_ipaddr(),
            hostname_ip_map: HashMap::new(),
        }
    }

    fn push_map(&mut self, key: String, value: String) {
        self.hostname_ip_map.insert(key, value);
    }

    fn create() -> File {
        let path = env::var("HOME").unwrap() + "/.config/telex.json";
        let file = File::create(path).unwrap();
        file
    }

    fn update_file(&mut self, mut file: File) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        file.write(data.as_bytes()).unwrap();
    }

    fn read_file() -> Self {
        let data = fs::read_to_string(env::var("HOME").unwrap() + "/.config/telex.json").unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

#[test]
fn test() {
    let data = Config::read_file();
    println!("{:?}", data.hostname_ip_map);
}
