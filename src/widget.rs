use crate::Cache;

pub trait WidgetRenderer<T> {
    fn get_data(&self) -> Result<T, Box<dyn std::error::Error>>;

    fn render_content(&self, data: T) -> Option<String>;

    fn render_error(&self, error: Box<dyn std::error::Error>) -> String {
        return String::from(error.to_string());
    }
}

pub struct Widget<T> {
    renderer: Box<dyn WidgetRenderer<T>>,
    max_width: Option<usize>,
    enabled: bool,
    fgf: Box<dyn Fn(&T) -> String>,
    bgf: Box<dyn Fn(&T) -> String>,
}

impl<T> Widget<T> {
    pub fn new(renderer: Box<dyn WidgetRenderer<T>>) -> Widget<T> {
        Widget {
            renderer,
            max_width: None,
            enabled: true,
            fgf: Box::new(|_| "default".to_string()),
            bgf: Box::new(|_| "default".to_string()),
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Widget<T> {
        self.enabled = enabled;
        self
    }

    pub fn fg(mut self, fg: &str) -> Widget<T> {
        let fg = fg.to_string();
        self.fgf = Box::new(move |_| fg.clone());
        self
    }

    pub fn max_width(mut self, width: usize) -> Widget<T> {
        self.max_width = Some(width);
        self
    }

    pub fn fg_func(mut self, fg: impl Fn(&T) -> String + 'static) -> Widget<T> {
        self.fgf = Box::new(fg);
        self
    }

    pub fn bg(mut self, bg: &str) -> Widget<T> {
        let bg = bg.to_string();
        self.bgf = Box::new(move |_| bg.clone());
        self
    }

    pub fn bg_func(mut self, bg: impl Fn(&T) -> String + 'static) -> Widget<T> {
        self.bgf = Box::new(bg);
        self
    }

    pub fn display(&self) -> Option<String> {
        if !self.enabled {
            return None;
        }

        match self.renderer.get_data() {
            Ok(data) => {
                let fg = (self.fgf)(&data);
                let bg = (self.bgf)(&data);

                let content = self.renderer.render_content(data);

                if let Some(content) = content {
                    let content = if let Some(max_width) = self.max_width {
                        if max_width < content.len() {
                            format!("{}...", &content[..max_width])
                        } else {
                            content
                        }
                    } else {
                        content
                    };

                    return Some(format!("#[fg={},bg={}] {} ", fg, bg, content));
                }

                None
            }
            Err(err) => Some(format!(
                "#[fg=#ffffff,bg=#ff0000]{}",
                self.renderer.render_error(err)
            )),
        }
    }
}
// impl<T> Widget<T> {
//     pub fn combine(widgets: Vec<Box<dyn AnyWidget>>) {
//         let mut final_string = String::from("");
//
//         for widget in widgets {
//             final_string += widget.display();
//         }
//     }
// }
