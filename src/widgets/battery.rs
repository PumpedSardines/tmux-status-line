use battery::units::ratio::percent;
use battery::{Battery, Manager};
use std::{error::Error, fmt};
use tmux_status_line::WidgetRenderer;

#[derive(Debug)]
pub struct NoBatteryError;
impl fmt::Display for NoBatteryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address is localhost")
    }
}
impl Error for NoBatteryError {}
pub fn get_battery() -> Result<Battery, Box<dyn Error>> {
    for battery in Manager::new()?.batteries()? {
        if let Ok(battery) = battery {
            return Ok(battery);
        }
    }

    Err(NoBatteryError.into())
}

pub struct BatteryWidget {}
impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget {}
    }
}
impl WidgetRenderer<BatteryData> for BatteryWidget {
    fn get_data(&self) -> Result<BatteryData, Box<dyn std::error::Error>> {
        let battery = get_battery()?;

        let is_charging = battery.time_to_full().is_some();
        let percentage = battery.state_of_charge().get::<percent>();
        let percentage = percentage.round() as usize;

        Ok(BatteryData {
            is_charging,
            percentage,
        })
    }

    fn render_content(&self, value: BatteryData) -> Option<String> {
        let mut battery_text = format!("{}%", value.percentage);

        if value.is_charging {
            battery_text = format!("{} {}", "\u{f0e7}", battery_text);
        }

        Some(battery_text)
    }
}

pub struct BatteryData {
    pub is_charging: bool,
    pub percentage: usize,
}
