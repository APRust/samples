use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::json;
use serde_json::Map;
use serde_json::Value;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();

    let mut data = "".to_string();
    file.read_to_string(&mut data).unwrap();
    let mut json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write file");
}