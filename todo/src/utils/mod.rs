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
