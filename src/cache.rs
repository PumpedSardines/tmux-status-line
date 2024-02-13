use std::path::PathBuf;

use serde::de::DeserializeOwned;

pub struct Cache {
    path: PathBuf,
    dur_time_path: PathBuf,
    duration: i64,
}

impl Cache {
    pub fn new<T>(name: T, duration: i64) -> Self
    where
        T: std::fmt::Display,
    {
        let current_dir = std::env::temp_dir();
        let path = current_dir.join(format!("tmux_status_line.pumpedsardines.{}.json", name));
        let dur_time_path =
            current_dir.join(format!("tmux_status_line.pumpedsardines.time.{}.txt", name));

        Self {
            path,
            duration,
            dur_time_path,
        }
    }
}

impl Cache {
    pub fn load<T>(&self) -> Option<T>
    where
        T: DeserializeOwned + Clone + serde::Serialize,
    {
        let dur_time = std::fs::read_to_string(&self.dur_time_path).ok();

        if dur_time.is_none() {
            return None;
        }
        let dur_time = dur_time.unwrap();
        let time = dur_time.parse::<i64>().ok();
        if time.is_none() {
            return None;
        }
        let time = time.unwrap();

        if time < chrono::Utc::now().timestamp() {
            return None;
        }

        let data = std::fs::read_to_string(&self.path).ok();
        if data.is_none() {
            return None;
        }
        let data = data.unwrap();

        let v = serde_json::from_str::<T>(&data).ok();

        if v.is_none() {
            std::fs::remove_file(&self.path).ok();
        }

        v.clone()
    }

    pub fn save<T>(&self, data: T)
    where
        T: DeserializeOwned + Clone + serde::Serialize,
    {
        let data = serde_json::to_string(&data).unwrap();
        std::fs::write(
            &self.dur_time_path,
            format!("{}", chrono::Utc::now().timestamp() + self.duration),
        )
        .ok();
        std::fs::write(&self.path, data).ok();
    }
}
