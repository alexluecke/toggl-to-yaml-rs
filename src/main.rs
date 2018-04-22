#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;
extern crate getopts;
extern crate csv;

use getopts::Options;
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
    time: Time
}

#[derive(Debug, Serialize, Deserialize)]
struct Time {
    hours: i32,
    minutes: i32
}

fn main() {
    let mut date_map = HashMap::<String, Vec<Bucket>>::new();
    let mut i = 0;

    for record in get_records(&get_options()).unwrap() {
        let hours_minutes: Vec<i32> = match record.duration {
            Some(v) => {
                v.split(":").take(2).map(|e| e.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            },
            None => {
                print_msg("Duration", i);
                std::process::exit(1);
            }
        };

        if hours_minutes.len() != 2 {
            print_msg("Duration", i);
            std::process::exit(1);
        }

        let time_bucket = Bucket {
            ticket: record.description.unwrap(),
            time: Time {
                hours: hours_minutes[0],
                minutes: hours_minutes[1]
            }
        };

        match record.start_date {
            Some(key) => match date_map.entry(key) {
                Entry::Occupied(mut entry) => { entry.get_mut().push(time_bucket); },
                Entry::Vacant(entry) => { entry.insert(vec![time_bucket]); }
            },
            None => {
                print_msg("Start Date", i); std::process::exit(1);
            }
        }

        i += 1;
    }

    println!("{}", serde_yaml::to_string(&date_map).unwrap());
}

fn get_options() -> String {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    let mut opts = Options::new();

    opts.optopt("f", "file", "toggl csv file", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(program, opts);
        std::process::exit(0);
    }

    match matches.opt_str("file") {
        Some(file) => { file },
        None => {
            print_usage(program, opts);
            std::process::exit(1);
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn get_records(file: &str) -> Result<Vec<TogglRecord>, Box<Error>> {
    let reader = fs::File::open(file).expect("File not found.");
    Ok(csv::Reader::from_reader(reader).deserialize().filter_map(|r| r.ok()).collect())
}

fn print_msg(field: &str, row: i32) {
    println!("Could not parse field {} for row {}.", field, row);
}
