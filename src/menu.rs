use crate::admin;
use crate::cli;
use crate::study;
use crate::util;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn main() {
    cli::clear_screen();
    cli::print_header();
    println!("{}", "What do you want to do?".yellow().bold());
    let selections = &["Study", "Add a new Deck", "Remove Deck", "Quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Study" => print_study_menu(),
        "Quit" => cli::clean_exit(),
        "Add a new Deck" => add_new_deck(),
        "Remove Deck" => remove_deck(),
        _ => println!(
            "{} {}",
            selections[selection].magenta(),
            "is not yet implemented".yellow().bold()
        ),
    }
}
fn print_study_menu() {
    cli::clear_screen();
    cli::print_header();
    let decks = util::get_decks();
    if decks.len() == 0 {
        println!(
            "{}",
            "You have no decks yet, please add one first.".red().bold()
        );
        back_or_quit();
    }
    let mut deck_names: Vec<String> = Vec::new();
    for deck in &decks {
        deck_names.push(deck.name.clone());
    }
    println!("{}", "\nWhat do you want to study today?".yellow().bold());
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&deck_names[..])
        .interact()
        .unwrap();

    let d = &decks
        .into_iter()
        .find(|d| d.name == deck_names[selection])
        .unwrap();
    study::iter_cards(d);
    main();
}
fn add_new_deck() {
    cli::clear_screen();
    cli::print_header();
    admin::add_new_deck();
    main();
}
fn remove_deck() {
    cli::clear_screen();
    cli::print_header();
    let decks = util::get_decks();
    if decks.len() == 0 {
        println!("{}", "You have no decks yet".red().bold());
        back_or_quit();
    }
    let mut deck_names: Vec<String> = Vec::new();
    for deck in &decks {
        deck_names.push(deck.name.clone());
    }
    println!("{} ({})", "\nWhich deck do you want to remove?".yellow().bold(), "ctrl + c to abort!".magenta().bold());
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&deck_names[..])
        .interact()
        .unwrap();

    let deck_name = &decks[selection].name;
    admin::remove_deck(&deck_name);
    main();
}
fn back_or_quit() {
    let selections = &["Back", "Quit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selections[selection] {
            "Back" => main(),
            "Quit" => cli::clean_exit(),
            _ => cli::clean_exit()
        }
}