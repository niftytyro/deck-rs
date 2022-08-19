use crate::app::{App, Slide};
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::{backend::Backend, Frame};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let current_slide = &app.slides[app.current_idx];
    render_slide(f, current_slide);
}

pub fn render_slide<B: Backend>(f: &mut Frame<B>, slide: &Slide) {
    let style = match slide.bg_color {
        Some(color) => Style::default().bg(color),
        None => Style::default(),
    };

    let parent_layout = Rect::new(f.size().x, f.size().y, f.size().width, f.size().height);

    f.render_widget(Block::default().style(style), parent_layout);

    let layout = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Min(80)])
        .horizontal_margin(20)
        .vertical_margin(5)
        .split(parent_layout);

    let title = Spans::from(Span::styled(
        &slide.title,
        Style::default().add_modifier(Modifier::UNDERLINED),
    ));

    let header = Paragraph::new(title)
        .block(Block::default().style(style))
        .style(Style::default().fg(Color::White))
        .alignment(tui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });

    let text: Vec<Spans> = slide
        .content
        .split("\n")
        .map(|line| Spans::from(line))
        .collect();

    let content = Paragraph::new(text)
        .block(Block::default().style(style))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(header, layout[0]);

    f.render_widget(content, layout[1]);
}
