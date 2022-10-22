use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use crate::model;
use crate::cli;

pub fn iter_cards(deck: &model::Deck) {
    let mut cards_next_iter: Vec<model::Card> = Vec::new();
    let total_number_of_cars = &deck.cards.len();
    for card in &deck.cards {
        cli::clear_screen();
        cli::print_header();
        let index = &deck.cards.iter().position(|c| c == card).unwrap();
        let number = index + 1;
        print_card(card, &number, total_number_of_cars, &deck.name);

        println!("Did you know that?");
        let selections = &["Yes", "No"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selections[selection] {
            "No" => {
                cards_next_iter.push(card.clone());
            }
            _ => continue,
        }
    }
    if cards_next_iter.len() > 0 {
        cli::clear_screen();
        cli::print_header();
        println!("{} {} {}","You did not know the answer for".yellow().bold(),cards_next_iter.len().to_string().magenta().bold(),"cards. Do you want to study those again?".yellow().bold());
        let selections = &["Yes", "No"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selections[selection] {
            "Yes" => {
                iter_cards(&model::Deck {
                    name: deck.name.clone() + " repetition",
                    cards: cards_next_iter,
                });
            }
            _ => return,
        }
    } else {
        return;
    }
    return;
}

fn print_card(card: &model::Card, number: &usize, total: &usize, deck_name: &str) {
    println!(
        "\n{} -> {} - Card ({}/{})\n{}\n",
        "Study".yellow().bold(),
        deck_name.yellow().bold(),
        number,
        total,
        card.front.magenta().bold()
    );
    
    let selections = &["Flip card"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Flip card" => println!("{}\n", card.back.green().bold()),
        _ => println!("{} is not yet implemented", selections[selection]),
    }
}

