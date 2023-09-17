use crossterm::{
    cursor,
    event::{self, KeyEventKind},
    execute,
    style::Stylize,
    terminal::{Clear, ClearType},
};
use std::io;

const INTERLUDE_STR: &str = r"
/ / / / / / / / / / / / / / / / / / / / / / / / /

            PRESS ANY KEY TO CONTINUE            

/ / / / / / / / / / / / / / / / / / / / / / / / /
";

pub fn lexicon_choose_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        "Failed to choose a word from the lexicon",
    )
}

pub fn interlude() -> io::Result<()> {
    println!("{}", INTERLUDE_STR.dark_yellow());
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Ok(event::Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    break;
                }
            }
        }
    }
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

pub fn pause() -> io::Result<()> {
    println!("{}", "Press any key to continue...".dark_grey());
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Ok(event::Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    break;
                }
            }
        }
    }
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

pub fn sleep(time: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(time));
}
