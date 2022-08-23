use crate::slide::{Slide, SlideNode, SlideNodeModifier, TextNode};

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use std::vec;
use tui::style::Color;

fn iter_nodes<'a>(node: &'a AstNode<'a>, slide_node: &mut SlideNode) -> Vec<TextNode> {
    match &mut node.data.borrow_mut().value {
        &mut NodeValue::Text(ref mut text) => {
            let mut text_node = TextNode::new();
            text_node.set_text(text);

            vec![text_node]
        }
        &mut NodeValue::Strong => {
            let mut modified_text_nodes = vec![];
            for each in node.children() {
                let text_nodes = iter_nodes(each, slide_node);
                for mut text_node in text_nodes {
                    text_node.add_modifier(SlideNodeModifier::BOLD);
                    modified_text_nodes.push(text_node);
                }
            }

            modified_text_nodes
        }
        &mut NodeValue::Emph => {
            let mut modified_text_nodes = vec![];
            for each in node.children() {
                let text_nodes = iter_nodes(each, slide_node);
                for mut text_node in text_nodes {
                    text_node.add_modifier(SlideNodeModifier::ITALIC);
                    modified_text_nodes.push(text_node);
                }
            }

            modified_text_nodes
        }
        &mut NodeValue::Strikethrough => {
            let mut modified_text_nodes = vec![];
            for each in node.children() {
                let text_nodes = iter_nodes(each, slide_node);
                for mut text_node in text_nodes {
                    text_node.add_modifier(SlideNodeModifier::STRIKETHROUGH);
                    modified_text_nodes.push(text_node);
                }
            }

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

pub fn generate_slides() {
    let input = "Hello, **world\nworldie**!";

    let slide = generate_slide(input);
}
