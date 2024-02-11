mod battery;
mod harvest;
mod misc;
mod widget;

use std::fs;

use widget::Widget;

macro_rules! w {
    ($w:expr) => {
        Widget::new(Box::new($w))
    };
}

macro_rules! widgets {
    ($($ws:expr),+) => {
        let v = vec![
            $(
                $ws.display(),
            )+
        ];

        let widget_contents = v
            .into_iter()
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<String>>()
            .join("");

        print!("{}", widget_contents);
    };
}

fn main() {
    let current_dir = homedir::get_my_home().unwrap().unwrap();
    let path = current_dir.join(".harvest");

    let data = fs::read_to_string(path).ok();
    #[derive(serde::Deserialize)]
    struct Credentials {
        username: String,
        password: String,
    }
    let credentials = data.map(|d| serde_json::from_str::<Credentials>(&d).unwrap());

    widgets![
        w!(harvest::HarvestWidget::new(
            credentials
                .as_ref()
                .map(|c| c.username.clone())
                .unwrap_or("".to_string()),
            credentials
                .as_ref()
                .map(|c| c.password.clone())
                .unwrap_or("".to_string())
        ))
        .enabled(credentials.is_some()),
        w!(misc::UptimeWidget::new()),
        w!(battery::BatteryWidget::new())
            .fg("#000000")
            .bg_func(|b| {
                match b.is_charging {
                    true => "#DFA000",
                    false => {
                        if b.percentage <= 20 {
                            "#F85552"
                        } else {
                            "#8DA101"
                        }
                    }
                }
                .to_string()
            }),
        w!(misc::DateWidget::new_from_pattern("%a %H:%M %Y-%m-%d"))
    ];
}
