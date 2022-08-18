use super::deck;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Print},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Result,
};
use std::io::{self, Write};

const ERROR_MESSAGE: &str = "Something went wrong";

pub fn run() {
    let mut w = io::stdout();
    let deck = deck::Deck {
        slides: vec![deck::Slide {
            headline: String::from("The Joy of Engineering"),
            content: String::from(
                "Presented by Udasi\ngithub - niftytyro\ntwitter - niftytyro\nblog - niftytyro.me",
            ),
        }],
        current_idx: 0,
    };

    execute!(w, terminal::EnterAlternateScreen).expect(ERROR_MESSAGE);

    enable_raw_mode().expect(ERROR_MESSAGE);

    render_slide(&mut w, &deck.slides[deck.current_idx]);

    execute!(w, style::ResetColor).expect(ERROR_MESSAGE);

    disable_raw_mode().expect(ERROR_MESSAGE);
}

pub fn render_slide<W>(w: &mut W, current_slide: &deck::Slide)
where
    W: Write,
{
    let (width, height) = terminal::size().expect(ERROR_MESSAGE);

    loop {
        let mut counter = 0;
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, (height / 10) + counter),
        )
        .expect(ERROR_MESSAGE);

        w.flush().expect(ERROR_MESSAGE);

        let headline = &current_slide.headline;
        let whitespace_len: usize = (width - (headline.len() as u16)).into();
        let whitespace = " ".repeat(whitespace_len / 2);

        let headline = String::from(format!("{}{}{}", whitespace, headline, whitespace));

        counter += 3;

        execute!(
            w,
            Print(headline),
            cursor::MoveTo(width / 20, (height / 10) + counter)
        )
        .expect(ERROR_MESSAGE);

        counter += 3;

        for line in current_slide.content.split("\n") {
            execute!(
                w,
                Print("\n"),
                Print(line),
                cursor::MoveTo(width / 20, (height / 10) + counter)
            )
            .expect(ERROR_MESSAGE);
            counter += 1;
        }

        match read_char().expect(ERROR_MESSAGE) {
            'q' => break,
            _ => {}
        };
    }
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
