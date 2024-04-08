use std::io::Result;
use std::io::stdout;
use crossterm::{
    event, ExecutableCommand,
    event::{KeyCode, read},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,}
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    loop {
        if let event::Event::Key(key) = read()? {
            println!("Pressed key: {:?}\r", key.code);
            if key.code == KeyCode::Esc {
                break;
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
