#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

extern crate csv;

use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;
use std::collections::hash_map::{Entry};
use std::fs;
use std::error::Error;

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
    let file = fs::File::open("./files/toggl.csv").expect("File not found.");
    Ok(csv::Reader::from_reader(file).deserialize().filter_map(|r| r.ok()).collect())
}

fn print_msg(field: &str, row: i32) {
    println!("Could not parse field {} for row {}.", field, row);
}
