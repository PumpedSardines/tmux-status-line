pub struct HarvestWidget {
    api: harvest::Api,
}
impl<'a> HarvestWidget {
    pub fn new(username: String, password: String) -> Self {
        Self {
            api: harvest::Api::new(username, password),
        }
    }
}
impl<'a> super::widget::WidgetRenderer<Option<harvest::RunningTimerInfo>> for HarvestWidget {
    fn get_data(&self) -> Result<Option<harvest::RunningTimerInfo>, Box<dyn std::error::Error>> {
        let resp: Option<harvest::RunningTimerInfo> = self.api.running_timer()?.into();

        Ok(resp)
    }

    fn render_content(&self, value: Option<harvest::RunningTimerInfo>) -> Option<String> {
        if let Some(data) = value {
            return Some(match data.notes {
                None => format!("\u{f017} {} - {}", data.hours, data.project),
                Some(notes) => format!("\u{f017} {} - {} - {}", data.hours, notes, data.project),
            });
        }

        None
    }
}
