/*
*   Ejercicio 2:
*       1. Raw mode.
*       2. Imprimir abecedario seguido.
*       3. Borrar lo imprimido.
* */

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
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

    for i in 1..=2 {
        for letter in 'A'..='Z' {
            print!("{}", letter);
            stdout.flush()?;
        }

        execute!(stdout, MoveToNextLine(1))?;
    }

    sleep(Duration::from_secs(3));
    execute!(stdout, Clear(ClearType::All))?;
    sleep(Duration::from_secs(2));

    disable_raw_mode()
}
