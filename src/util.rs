use std::fs;

use crate::{cli, model};

pub fn get_decks() -> Vec<model::Deck> {
    let path = cli::ensure_home_dir();
    let mut decks = Vec::new();
    let read_dir_result = fs::read_dir(path);
    if read_dir_result.is_err() {
        return decks;
    }
    let files = read_dir_result.unwrap();
    for file in files {
        let data = fs::read_to_string(file.unwrap().path()).expect("Unable to read file");
        let deck:model::Deck = serde_json::from_str(&data).unwrap();
        decks.push(deck)
    }
    return decks;
}