/*
*   Ejercicio 2:
*       1. Enable raw mode.
*       2. Print the alphabet in a line (x2).
*       3. Disable raw mode.
* */

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::Duration,
};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;

    for _ in 1..=2 {
        for letter in 'A'..='Z' {
            //Option 1
            //print!("{letter}");

            //Option 2
            execute!(stdout, Print(letter))?;

            stdout.flush()?;
        }

        execute!(stdout, MoveToNextLine(1))?;
    }

    sleep(Duration::from_secs(3));
    execute!(stdout, Clear(ClearType::All))?;
    sleep(Duration::from_secs(2));

    disable_raw_mode()
}
