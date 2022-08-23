use std::fmt;
use tui::{layout::Alignment, style::Color};

#[derive(Debug)]
pub enum SlideNodeModifier {
    BOLD,
    ITALIC,
    STRIKETHROUGH,
    UNDERLINE,
}

pub struct Slide {
    pub nodes: Vec<SlideNode>,
    pub bg_color: Option<Color>,
    pub fg_color: Option<Color>,
}

pub struct SlideNode {
    pub text_nodes: Vec<TextNode>,
    pub alignment: Alignment,
}

pub struct TextNode {
    pub text: String,
    pub modifiers: Vec<SlideNodeModifier>,
}

impl Slide {
    pub fn new(bg_color: Option<Color>, fg_color: Option<Color>) -> Slide {
        Slide {
            nodes: vec![],
            bg_color,
            fg_color,
        }
    }

    pub fn add_node(self: &mut Self, node: SlideNode) {
        self.nodes.push(node);
    }
}

impl fmt::Debug for Slide {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.nodes)
    }
}

impl fmt::Debug for SlideNode {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.text_nodes)
    }
}

impl fmt::Debug for TextNode {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "text: {} || modifiers:{:?}", self.text, self.modifiers)
    }
}

impl SlideNode {
    pub fn new() -> SlideNode {
        SlideNode {
            text_nodes: vec![],
            alignment: Alignment::Left,
        }
    }

    pub fn set_text_nodes(self: &mut Self, text_nodes: Vec<TextNode>) {
        self.text_nodes = text_nodes;
    }
}

impl TextNode {
    pub fn new() -> TextNode {
        TextNode {
            text: String::from(""),
            modifiers: vec![],
        }
    }
}
