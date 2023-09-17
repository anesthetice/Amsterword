use std::io;
use rand::{
    Rng,
    thread_rng,
    seq::SliceRandom
};
use crossterm::{
    execute,
    cursor,
    terminal::{Clear, ClearType},
    style::Stylize
};

mod config;
use config::Config;
mod utils;
use utils::*;

fn main() -> io::Result<()> {
    let config: Config = Config::load_from_file("./config.json")?;
    let mut rng = thread_rng();
    
    let generic_word: String = config.lexicon.choose(&mut rng).ok_or(lexicon_choose_error())?.clone();
    let anomaly_word: String = {
        let mut word: String = config.lexicon.choose(&mut rng).ok_or(lexicon_choose_error())?.clone();
        while word == generic_word {
            word = config.lexicon.choose(&mut rng).ok_or(lexicon_choose_error())?.clone()
        }
        word
    };
    
    let mut list_of_words: Vec<String> = vec![generic_word.clone(); config.number_of_players-1];
    list_of_words.push(anomaly_word.clone());

    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    interlude()?;

    for num in 0..config.number_of_players {
        let selected_word: String = list_of_words.choose(&mut rng).unwrap().clone();

        if selected_word == generic_word {
            list_of_words.remove(0);
            println!("-------------- PLAYER {} --------------\n\nyour word is : '{}'\n", num, if config.hint_system {if rng.gen_bool(config.hint_probability_generic) {selected_word.red().bold()} else {selected_word.green().bold()}} else {selected_word.cyan().bold()});
            pause()?;
            sleep(0.2);
            interlude()?;
        } else {
            list_of_words.pop();
            println!("-------------- PLAYER {} --------------\n\nyour word is : '{}'\n", num, if config.hint_system {if rng.gen_bool(config.hint_probability_anomaly) {selected_word.red().bold()} else {selected_word.green().bold()}} else {selected_word.cyan().bold()});
            pause()?;
            sleep(0.2);
            interlude()?;
        }

    }
    Ok(())
}
