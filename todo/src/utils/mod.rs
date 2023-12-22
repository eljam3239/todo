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
