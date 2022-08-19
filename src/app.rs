use tui::style::Color;

pub struct Slide {
    pub title: String,
    pub content: String,
    pub bg_color: Option<Color>,
}

pub struct App {
    pub slides: Vec<Slide>,
    pub current_idx: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(slides: Vec<Slide>) -> App {
        App {
            slides,
            current_idx: 0,
            should_quit: false,
        }
    }

    pub fn on_key(self: &mut Self, key: char) {
        match key {
            'q' => self.should_quit = true,
            _ => (),
        };
    }
}
