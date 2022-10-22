use crate::cli;
use crate::model;
use colored::*;


pub fn add_new_deck() {
    let deck_name = get_deck_name();
    let mut deck: model::Deck = model::Deck {
        name: deck_name,
        cards: Vec::new(),
    };

    let path = cli::ensure_home_dir();
    let file = &format!("{}{}.flashcards.json", path, deck.name);

    // check if file exist
    if std::path::Path::new(file).exists() {
        println!("{}{}", file, " already exists, try again".red().bold());
        return add_new_deck();
    }

    let json = serde_json::to_string(&deck).unwrap();
    std::fs::write(file, json).unwrap();

    loop {
        println!(
            "{}",
            "What do you want on the front of the card?".yellow().bold()
        );
        let mut card_front = String::new();
        std::io::stdin().read_line(&mut card_front).unwrap();
        card_front = card_front.trim().to_string();

        println!(
            "{}",
            "What do you want on the back of the card?".yellow().bold()
        );
        let mut card_back = String::new();
        std::io::stdin().read_line(&mut card_back).unwrap();
        card_back = card_back.trim().to_string();
        let card: model::Card = model::Card {
            front: card_front,
            back: card_back,
        };
        deck.cards.push(card);
        let json = serde_json::to_string(&deck).unwrap();
        std::fs::write(file, json).unwrap();
        println!(
            "{}",
            "Do you want to add another card? (y/n)".yellow().bold()
        );
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        answer = answer.trim().to_string();
        if answer == "n" {
            break;
        }
    }
}

pub fn remove_deck(deck_name: &String) {
    let path = cli::ensure_home_dir();
    let file = &format!("{}{}.flashcards.json", path, deck_name);
    if std::path::Path::new(file).exists() {
        std::fs::remove_file(file).unwrap();
        println!("{}{}", file, " removed".green().bold());
    } else {
        println!("{}{}", file, " does not exist".red().bold());
    }
}
fn get_deck_name() -> String {
    println!("{} ({})", "\nPlease enter the name of the new deck".yellow().bold(), "ctrl + c to abort!".magenta().bold());
    let mut deck_name = String::new();
    std::io::stdin().read_line(&mut deck_name).unwrap();
    deck_name = deck_name.trim().to_string();
    if deck_name.len() == 0 {
        println!("{}", "Deck name cannot be empty.".red().bold());
        return get_deck_name();
    } else {
        return deck_name;
    }
}
