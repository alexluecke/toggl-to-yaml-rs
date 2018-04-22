#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;
extern crate getopts;
extern crate csv;

use getopts::Options;
use serde::ser::Serialize;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug,Deserialize)]
struct TogglRecord {
    #[serde(rename(deserialize = "Description"))]
    description: Option<String>,
    #[serde(rename(deserialize = "Start date"))]
    start_date: Option<String>,
    #[serde(rename(deserialize = "Duration"))]
    duration: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bucket {
    ticket: String,
    time: String
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DateMap {
    data: HashMap<String, Vec<Bucket>>
}

fn main() {
    let mut date_map = DateMap::default();
    let mut i = 0;

    for record in get_records().unwrap() {
        let hour_minute: Vec<i32> = match record.duration {
            Some(v) => {
                v.split(":").take(2).map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            },
            None => {
                print_msg("Duration", i);
                std::process::exit(1);
            }
        };

        if hour_minute.len() != 2 {
            print_msg("Duration", i);
            std::process::exit(1);
        }

        let time_bucket = Bucket {
            ticket: record.description.unwrap(),
            time: format!("{}h {}m", hour_minute[0], hour_minute[1])
        };

        match record.start_date {
            Some(key) => match date_map.data.entry(key) {
                Entry::Occupied(mut entry) => { entry.get_mut().push(time_bucket); },
                Entry::Vacant(entry) => { entry.insert(vec![time_bucket]); }
            },
            None => {
                print_msg("Start Date", i); std::process::exit(1);
            }
        }

        i += 1;
    }

    println!("{}", serde_yaml::to_string(&date_map.data).unwrap());
}

fn get_records() -> Result<Vec<TogglRecord>, Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("f", "file", "toggl csv file", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    match matches.opt_str("file") {
        Some(file) => {
            let reader = fs::File::open(file).expect("File not found.");
            Ok(csv::Reader::from_reader(reader).deserialize().filter_map(|r| r.ok()).collect())
        }
        None => {
            print_usage(&program, opts);
            std::process::exit(1);
        }
    }
}

fn print_msg(field: &str, row: i32) {
    println!("Could not parse field {} for row {}.", field, row);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
