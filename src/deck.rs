pub struct Deck {
    pub slides: Vec<Slide>,
    pub current_idx: usize,
}

pub struct Slide {
    pub headline: String,
    pub content: String,
}
