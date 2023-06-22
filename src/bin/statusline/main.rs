use harvest::widget::Widget as HarvestWidget;
use widget::battery::BatteryWidget;
use widget::misc::*;
use widget::spotify::SpotifyWidget;
use widget::widget::Widget;

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
        w!(SpotifyWidget::new())
            .fg("#ffffff")
            .bg("#125724")
            .max_width(50),
        w!(HarvestWidget::new())
            .fg("#ffffff")
            .bg("#c74900")
            .max_width(50),
        w!(UptimeWidget::new()),
        w!(BatteryWidget::new()).fg("#000000").bg_func(|b| {
            match b.is_charging {
                true => "#ffee00",
                false => "#51c449",
            }
            .to_string()
        }),
        w!(DateWidget::new_from_pattern("%a %H:%M %Y-%m-%d"))
    ];
}
