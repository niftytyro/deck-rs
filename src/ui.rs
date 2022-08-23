use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Paragraph, Wrap};
use tui::{backend::Backend, Frame};

use crate::app::App;
use crate::slide::{Slide, SlideNodeModifier};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let current_slide = &app.slides[app.current_idx];
    render_slide(f, current_slide);
}

pub fn render_slide<B: Backend>(f: &mut Frame<B>, slide: &Slide) {
    let style = match slide.bg_color {
        Some(color) => Style::default().bg(color),
        None => Style::default(),
    };

    let nodes_len = slide.nodes.len();
    let constraints = vec![Constraint::Percentage(100 / (nodes_len as u16)); nodes_len];

    let parent_layout = Rect::new(f.size().x, f.size().y, f.size().width, f.size().height);

    f.render_widget(Block::default().style(style), parent_layout);

    let layout = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(constraints)
        .horizontal_margin(20)
        .vertical_margin(5)
        .split(parent_layout);

    for (index, node) in slide.nodes.iter().enumerate() {
        let text: Vec<Spans> = node
            .text_nodes
            .iter()
            .map(|text_node| {
                let mut style = Style::default();

                for modifier in &text_node.modifiers {
                    match modifier {
                        SlideNodeModifier::BOLD => {
                            style = style.add_modifier(Modifier::BOLD);
                        }
                        SlideNodeModifier::ITALIC => {
                            style = style.add_modifier(Modifier::ITALIC);
                        }
                        SlideNodeModifier::STRIKETHROUGH => {
                            style = style.add_modifier(Modifier::CROSSED_OUT);
                        }
                        SlideNodeModifier::UNDERLINE => {
                            style = style.add_modifier(Modifier::UNDERLINED);
                        }
                    }
                }

                Spans::from(Span::styled(&text_node.text, style))
            })
            .collect();

        let content = Paragraph::new(text)
            .block(Block::default().style(style))
            .style(Style::default().fg(Color::White))
            .alignment(node.alignment)
            .wrap(Wrap { trim: false });

        f.render_widget(content, layout[index]);
    }
}
