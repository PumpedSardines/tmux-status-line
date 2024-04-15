mod widgets;
use widgets::*;

use std::fs;

use tmux_status_line::Widget;

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
        w!(HarvestWidget::new(
            credentials
                .as_ref()
                .map(|c| c.username.clone())
                .unwrap_or("".to_string()),
            credentials
                .as_ref()
                .map(|c| c.password.clone())
                .unwrap_or("".to_string())
        ))
        .fg("#ffffff")
        .bg("#c74900")
        .max_width(50)
        .enabled(credentials.is_some()),
        w!(UptimeWidget::new()),
        w!(BatteryWidget::new()).fg("#000000").bg_func(|b| {
            match b.is_charging {
                true => "#ffff00",
                false => {
                    if b.percentage <= 20 {
                        "#f85552"
                    } else {
                        "#8da101"
                    }
                }
            }
            .to_string()
        }),
        w!(DateWidget::new_from_pattern("%a %H:%M %Y-%m-%d"))
    ];
}
