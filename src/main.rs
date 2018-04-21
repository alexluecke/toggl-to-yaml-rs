#[macro_use]
extern crate serde_derive;

extern crate csv;

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

fn main() {
    let what = example(); // owner
}

fn example() -> Result<Vec<TogglRecord>, Box<Error>> {
    let file = fs::File::open("./files/toggl.csv").expect("File not found.");
    Ok(csv::Reader::from_reader(file).deserialize().filter_map(|r| r.ok()).collect())
}
