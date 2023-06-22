use regex::Regex;
use std::process::Command;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SpotifyWidgetError;
impl fmt::Display for SpotifyWidgetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address is localhost")
    }
}
impl Error for SpotifyWidgetError {}

pub struct SpotifyWidget {}
impl SpotifyWidget {
    pub fn new() -> SpotifyWidget {
        SpotifyWidget {}
    }
}
impl super::widget::WidgetRenderer<Option<String>> for SpotifyWidget {
    #[cfg(target_os = "macos")]
    fn get_data(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let command = "
            tell application \"System Events\"
	            set process_list to (name of every process)
            end tell

            if process_list contains \"Spotify\" then
	            tell application \"Spotify\"
		            if player state is playing or player state is paused then

			            set track_name to name of current track
			            set artist_name to artist of current track

			            if player state is playing then
				            set now_playing to artist_name & \" - \" & track_name
			            end if
		            end if
	            end tell
            end if";

        let re = Regex::new(r"^\s+-?\s*|\s*-?\s+$").unwrap();

        let output = Command::new("osascript").arg("-e").arg(command).output()?;

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        let result = re.replace_all(&result, "");

        if result == "".to_string() {
            return Ok(None);
        }

        return Ok(Some(format!("\u{f001} {}", result)));
    }

    #[cfg(not(target_os = "macos"))]
    fn get_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        panic!("The spotify widget is only available for macos");
    }

    fn render_content(&self, value: Option<String>) -> Option<String> {
        value
    }
}
