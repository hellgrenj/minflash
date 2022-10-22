use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Card {
    pub front: String,
    pub back: String,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
}