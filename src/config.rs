use std::collections::HashMap;
use std::env;
use serde_yaml;

#[allow(dead_code)]
pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>
}


#[allow(dead_code)]
impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];
        let file = std::fs::File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
        return Config{map}
    }
}