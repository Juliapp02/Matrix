/*
    Exercise 1:
        1. Enable raw mode.
        2. Move cursor to the four corners of the terminal.
        3. Disable raw mode.

*/

use std::{io::Result, io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

fn main() -> Result<()> {
    let col_row = size().unwrap();

    enable_raw_mode()?;

    execute!(stdout(), Clear(ClearType::All))?;

    execute!(stdout(), MoveTo(0, 0))?;
    sleep(Duration::from_secs(2));
    execute!(stdout(), MoveTo(col_row.0 - 1, 0))?;
    sleep(Duration::from_secs(2));
    execute!(stdout(), MoveTo(0, col_row.1 - 1))?;
    sleep(Duration::from_secs(2));
    execute!(stdout(), MoveTo(col_row.0 - 1, col_row.1 - 1))?;
    sleep(Duration::from_secs(2));

    disable_raw_mode()
}
