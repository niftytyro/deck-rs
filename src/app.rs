use crate::slide::Slide;

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

    pub fn on_right(self: &mut Self) {
        if self.slides.len() - 1 > self.current_idx {
            self.current_idx += 1;
        }
    }

    pub fn on_left(self: &mut Self) {
        if self.current_idx > 0 {
            self.current_idx -= 1;
        }
    }
}
