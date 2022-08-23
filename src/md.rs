use crate::slide::{Slide, SlideNode, SlideNodeModifier, TextNode};
use crate::ERROR_MESSAGE;

use argh::FromArgs;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use std::{fs, vec};
use tui::layout::Alignment;
use tui::style::Color;

/// Presentations in the terminal
#[derive(FromArgs)]
struct Args {
    /// path to the .md file
    #[argh(option)]
    path: String,
}

fn modify_node<'a>(
    node: &'a AstNode<'a>,
    slide_node: &mut SlideNode,
    modifier: &dyn Fn(&mut TextNode) -> (),
) -> Vec<TextNode> {
    let mut modified_text_nodes = vec![];
    for each in node.children() {
        let text_nodes = iter_nodes(each, slide_node);
        for mut text_node in text_nodes {
            modifier(&mut text_node);
            modified_text_nodes.push(text_node);
        }
    }

    modified_text_nodes
}

fn iter_nodes<'a>(node: &'a AstNode<'a>, slide_node: &mut SlideNode) -> Vec<TextNode> {
    match &mut node.data.borrow_mut().value {
        &mut NodeValue::Text(ref mut text) => {
            let mut text_node = TextNode::new();
            text_node.text = String::from_utf8(text.clone()).expect(ERROR_MESSAGE);

            vec![text_node]
        }
        &mut NodeValue::Strong => {
            let modified_text_nodes = modify_node(node, slide_node, &|text_node: &mut TextNode| {
                text_node.modifiers.push(SlideNodeModifier::BOLD)
            });

            modified_text_nodes
        }
        &mut NodeValue::Emph => {
            let modified_text_nodes = modify_node(node, slide_node, &|text_node: &mut TextNode| {
                text_node.modifiers.push(SlideNodeModifier::ITALIC)
            });

            modified_text_nodes
        }
        &mut NodeValue::Strikethrough => {
            let modified_text_nodes = modify_node(node, slide_node, &|text_node: &mut TextNode| {
                text_node.modifiers.push(SlideNodeModifier::STRIKETHROUGH)
            });

            modified_text_nodes
        }
        &mut NodeValue::Heading(_) => {
            slide_node.alignment = Alignment::Center;
            let modified_text_nodes = modify_node(node, slide_node, &|text_node: &mut TextNode| {
                text_node.modifiers.append(&mut vec![
                    SlideNodeModifier::BOLD,
                    SlideNodeModifier::UNDERLINE,
                ]);
            });

            modified_text_nodes
        }
        _ => {
            let mut text_nodes = vec![];
            for each in node.children() {
                text_nodes.append(&mut iter_nodes(each, slide_node));
            }

            text_nodes
        }
    }
}

pub fn generate_slide(slide: &str) -> Slide {
    let arena = Arena::new();
    let root = parse_document(&arena, slide, &ComrakOptions::default());

    let mut slide = Slide::new(Some(Color::Black), Some(Color::White));

    for each in root.children() {
        let mut slide_node = SlideNode::new();
        let text_nodes = iter_nodes(each, &mut slide_node);
        slide_node.set_text_nodes(text_nodes);
        slide.add_node(slide_node);
    }

    slide
}

pub fn generate_slides() -> Vec<Slide> {
    let args: Args = argh::from_env();
    let input = fs::read_to_string(args.path).expect(ERROR_MESSAGE);

    let mut slides: Vec<Slide> = vec![];

    for slide in input.split("---").collect::<Vec<&str>>() {
        slides.push(generate_slide(slide));
    }

    slides
}
