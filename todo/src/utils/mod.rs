use crate::structs;
use chrono;
use colorize::*;
use rand::prelude::*;
use serde_json::from_str;
use serde_json::Result;
use std::{fs, io::Write};

pub fn init(){
    if !fs::metadata("C:\\.todobook").is_ok(){
        fs::create_dir("C:\\.todobook").unwrap();

        let mut file = fs::File::create(DATA_FILE).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();
        println!("{} {}", "Created folder and file".green(), DATA_FILE);
    }
    else if !fs::metadata(DATA_FILE).is_ok(){
        let mut file = fs::File::create(DATA_FILE).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "created file".green(), DATA_FILE);
    }
}

pub fn get_args() -> structs::Command {
    let args = std::env::args().collect::<Vec<String>>();
    let command = args.get(1).unwrap_or(&"".to_string()).to_string();
    let arguments = args.get(2).unwrap_or(&"".to_string()).to_string();

    structs::Command { command, arguments }
}

pub fn get_timestamp() -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%m-%d %H:%M").to_string();

    timestamp
}
pub fn get_id() -> u32 {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..1000);

    id + rng.gen_range(1..1000)
}

pub fn get_todos() -> Result<Vec<structs::Todo>> {
    let data = fs::read_to_string(DATA_FILE).unwrap();
    let todos: structs::ConfigFile = from_str(&data)?;

    Ok(todos.data)
}

pub fn save_todos(todos: Vec<structs::Todo>) {
    let config_file = structs::ConfigFile { data: todos };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create(DATA_FILE).unwrap();
    file.write_all(json.as_bytes()0.unwrap();
}
