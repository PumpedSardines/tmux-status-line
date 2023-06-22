use chrono;

pub struct UptimeWidget {}
impl UptimeWidget {
    pub fn new() -> UptimeWidget {
        UptimeWidget {}
    }
}

macro_rules! format_plural {
    ($s:expr, $p:expr, $d:expr) => {
        if $d == 1.0 {
            format!($s, $d)
        } else {
            format!($p, $d)
        }
    };
}

impl super::widget::WidgetRenderer<String> for UptimeWidget {
    fn get_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let uptime = uptime_lib::get()?;

        let minutes = uptime.as_secs_f64() / 60.0;
        if minutes < 60.0 {
            return Ok(format_plural!("{} minute", "{} minutes", minutes.round()));
        }

        let hours = minutes / 60.0;
        if hours < 24.0 {
            return Ok(format_plural!("{} hour", "{} hours", hours.round()));
        }

        let days = hours / 24.0;

        Ok(format_plural!("{} day", "{} days", days.round()))
    }

    fn render_content(&self, value: String) -> Option<String> {
        Some(value)
    }
}

pub struct DateWidget<'a> {
    pattern: &'a str,
}
impl<'a> DateWidget<'a> {
    pub fn new() -> DateWidget<'a> {
        DateWidget { pattern: "%H:%M" }
    }

    pub fn new_from_pattern(pattern: &'a str) -> DateWidget<'a> {
        DateWidget { pattern }
    }
}
impl<'a> super::widget::WidgetRenderer<String> for DateWidget<'a> {
    fn get_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let now = chrono::Local::now().naive_local();

        Ok(now.format(self.pattern).to_string())
    }

    fn render_content(&self, value: String) -> Option<String> {
        Some(value)
    }
}
