use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct Api {
    username: String,
    password: String,
}

impl Api {
    pub fn new(username: String, password: String) -> Api {
        Api { username, password }
    }

    pub fn running_timer(&self) -> Result<Option<RunningTimerInfo>, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get("https://api.harvestapp.com/v2/time_entries")
            .header("Authorization", format!("Bearer {}", self.password.clone()))
            .header("Harvest-Account-ID", self.username.clone())
            .header("User-Agent", "statusline daemon")
            .send()?;

        let text = response.text()?;
        let serialized: TimeEntriesResponse = serde_json::from_str(&text)?;

        for entry in serialized.time_entries {
            if entry.is_running {
                let re = regex::Regex::new(r"^\s*$").unwrap();

                let notes = match entry.notes {
                    None => None,
                    Some(notes) => match re.is_match(&notes) {
                        true => None,
                        false => Some(notes.replace("\n", "")),
                    },
                };

                return Ok(Some(RunningTimerInfo {
                    notes,
                    project: entry.project.name.replace("\n", ""),
                    hours: {
                        let time = entry.hours;
                        let hours = (time - (time % 1.0)).round();
                        let minutes = ((time - hours) * 60.0).round();
                        format!("{:0>2}:{:0>2}", hours as usize, minutes as usize)
                    },
                }));
            }
        }

        Ok(None)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RunningTimerInfo {
    pub notes: Option<String>,
    pub hours: String,
    pub project: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeEntriesResponse {
    time_entries: Vec<TimeEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeEntry {
    hours: f64,
    notes: Option<String>,
    project: Project,
    is_running: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
}
