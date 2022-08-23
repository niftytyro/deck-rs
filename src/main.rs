mod app;
mod md;
mod slide;
mod ui;

use std::io;

use app::App;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, disable_raw_mode};
use crossterm::{execute, terminal::enable_raw_mode};
use md::generate_slides;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

const ERROR_MESSAGE: &str = "Something went wrong";

pub fn run() {
    // Setup terminal
    enable_raw_mode().expect(ERROR_MESSAGE);
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen).expect(ERROR_MESSAGE);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect(ERROR_MESSAGE);

    let app = App::new(generate_slides());

    run_app(&mut terminal, app);

    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen).expect(ERROR_MESSAGE);
    terminal.show_cursor().expect(ERROR_MESSAGE);

    disable_raw_mode().expect(ERROR_MESSAGE);
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) {
    loop {
        terminal
            .draw(|f| ui::draw(f, &mut app))
            .expect(ERROR_MESSAGE);

        if let Event::Key(key) = event::read().expect(ERROR_MESSAGE) {
            match key.code {
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Right => app.on_right(),
                KeyCode::Left => app.on_left(),
                _ => {}
            }
        }
        if app.should_quit {
            return;
        }
    }
}

fn main() {
    run();
}
