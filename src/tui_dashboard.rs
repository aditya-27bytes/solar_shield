use std::io::{self, Write};
use std::io::Result;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};


pub fn show_dashboard(message: &str) -> std::io::Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
    println!("{}", message);
    Ok(())
}
