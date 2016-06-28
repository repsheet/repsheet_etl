use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::hash_map::Entry::{Vacant, Occupied};

#[derive(Default)]
pub struct Actor {
    request_count: i64,
    methods:       HashMap<String, i64>,
    responses:     HashMap<String, i64>,
}

struct LogEntry {
    address:  String,
    method:   String,
    response: String,
}

impl LogEntry {
    fn valid(&self) -> bool {
        if self.address.is_empty() || self.method.is_empty() || self.response.is_empty() {
            return false;
        }
        return true;
    }
}

fn create_or_increment(hash: &mut HashMap<String, i64>, key: String) {
    match hash.entry(key) {
        Vacant(e) => { e.insert(1); },
        Occupied(mut e) => { *e.get_mut() += 1 },
    }
}

fn process_line(actors: &mut HashMap<String, Actor>, line: &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    let log_entry = LogEntry { address: parts[0].to_string(), method: parts[5].replace("\"", "").to_string(), response: parts[8].to_string() };
    if log_entry.valid() {
        match actors.entry(log_entry.address) {
            Vacant(key) => {
                let mut actor = Actor { request_count: 1, ..Default::default() };
                actor.methods.insert(log_entry.method, 1);
                actor.responses.insert(log_entry.response, 1);
                key.insert(actor);
            },
            Occupied(mut actor) => {
                actor.get_mut().request_count += 1;
                create_or_increment(&mut actor.get_mut().methods, log_entry.method);
                create_or_increment(&mut actor.get_mut().responses, log_entry.response);
            },
        }
    } else {
        println!("Invalid");
    }
}

pub fn process(logfile: &str) -> Result<HashMap<String, Actor>, String> {
    let path = Path::new(logfile);
    let file_handle = match File::open(&path) {
        Err(why) => {
            return Err(format!("Could not open {} : {}", path.display(), Error::description(&why)));
        },
        Ok(file) => file,
    };

    let reader = BufReader::new(file_handle);
    let mut actors = HashMap::<String, Actor>::new();
    for line in reader.lines() {
        match line {
            Ok(line) => process_line(&mut actors, &line),
            Err(e)   => return Err(format!("ERROR {}", e)),
        }
    }

    return Ok(actors);
}
