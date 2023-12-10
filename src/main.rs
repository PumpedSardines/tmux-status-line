mod battery;
mod misc;
mod widget;

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
    widgets![
        w!(misc::UptimeWidget::new()),
        w!(battery::BatteryWidget::new())
            .fg("#000000")
            .bg_func(|b| {
                match b.is_charging {
                    true => "#ffee00",
                    false => "#51c449",
                }
                .to_string()
            }),
        w!(misc::DateWidget::new_from_pattern("%a %H:%M %Y-%m-%d"))
    ];
}
