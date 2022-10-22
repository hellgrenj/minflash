use colored::*;
pub fn clean_exit() {
    clear_screen();
    println!("{}", "See you later!".yellow().bold());
    std::process::exit(0);
}
pub fn clear_screen() {
    print!("{}[2J", 27 as char); // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // position cursor at row 1, col 1
}
pub fn ensure_home_dir() -> String {
    let home = dirs::home_dir().unwrap();
    let p = format!("{}/.minflash/decks/", home.display());
    std::fs::create_dir_all(&p)
        .expect("Could not create folders in home directory");
    return p;
}
pub fn print_header () {
    println!("{}", "..::M I N F L A S H::..".magenta().bold());
}