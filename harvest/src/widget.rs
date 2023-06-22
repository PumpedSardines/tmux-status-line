pub struct Widget {}
impl Widget {
    pub fn new() -> Widget {
        Widget {}
    }
}
impl widget::widget::WidgetRenderer<Option<super::api::RunningTimerInfo>> for Widget {
    fn get_data(&self) -> Result<Option<super::api::RunningTimerInfo>, Box<dyn std::error::Error>> {
        let resp: Option<super::api::RunningTimerInfo> = serde_json::from_str(
            &reqwest::blocking::get("http://127.0.0.1:57192/harvest")?.text()?,
        )?;

        Ok(resp)
    }

    fn render_content(&self, value: Option<super::api::RunningTimerInfo>) -> Option<String> {
        if let Some(data) = value {
            return Some(match data.notes {
                None => format!("\u{f017} {} - {}", data.hours, data.project),
                Some(notes) => format!("\u{f017} {} - {} - {}", data.hours, notes, data.project),
            });
        }

        None
    }
}
