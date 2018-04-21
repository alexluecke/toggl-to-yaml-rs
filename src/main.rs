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
    #[serde(rename(serialize = "user", deserialize = "User"))]
    user: Option<String>,
    #[serde(rename(serialize = "email", deserialize = "Email"))]
    email: Option<String>,
    #[serde(rename(serialize = "client", deserialize = "Client"))]
    client: Option<String>,
    #[serde(rename(serialize = "project", deserialize = "Project"))]
    project: Option<String>,
    #[serde(rename(serialize = "task", deserialize = "Task"))]
    task: Option<String>,
    #[serde(rename(serialize = "description", deserialize = "Description"))]
    description: Option<String>,
    #[serde(rename(serialize = "billable", deserialize = "Billable"))]
    billable: Option<String>,
    #[serde(rename(serialize = "start_date", deserialize = "Start date"))]
    start_date: Option<String>,
    #[serde(rename(serialize = "start_time", deserialize = "Start time"))]
    start_time: Option<String>,
    #[serde(rename(serialize = "end_date", deserialize = "End date"))]
    end_date: Option<String>,
    #[serde(rename(serialize = "end_time", deserialize = "End time"))]
    end_time: Option<String>,
    #[serde(rename(serialize = "duration", deserialize = "Duration"))]
    duration: Option<String>,
    #[serde(rename(serialize = "tags", deserialize = "Tags"))]
    tags: Option<String>,
    #[serde(rename(serialize = "amount", deserialize = "Amount ()"))]
    amount: Option<String>
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
            Some(v) => v.split(":")
                .take(2)
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            None => {
                println!("Error parsing duration field.");
                std::process::exit(1);
            }
        };

        if hour_minute.len() != 2 {
            println!("Duration field not formatted correctly for row: {}, value: {:?}", i, hour_minute);
            std::process::exit(1);
        }

        let what = Bucket {
            ticket: record.description.unwrap(),
            time: format!("{}h {}m", hour_minute[0], hour_minute[1])
        };

        match date_map.data.entry(record.start_date.unwrap()) {
            Entry::Occupied(mut entry) => { entry.get_mut().push(what); },
            Entry::Vacant(entry) => { entry.insert(vec![what]); }
        }

        i += 1;
    }

    println!("{}", serde_yaml::to_string(&date_map.data).unwrap());
}

fn get_records() -> Result<Vec<TogglRecord>, Box<Error>> {
    let file = fs::File::open("./files/toggl.csv").expect("File not found.");
    Ok(csv::Reader::from_reader(file).deserialize().filter_map(|r| r.ok()).collect())
}
