/*
*   Exercise 4*
*       1. Every 3 horizontal lines, delete the first one printed.
*       2. At the beginning, decide randomly (0.5% probability) which rows are going
*          to have info printed in them. (the rows always print
*          the vector with random characters).
*       3. When printing a row, reprint the previous one in White.
**/

use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

use rand::{Rng, thread_rng};

#[derive(PartialEq, Eq)]
enum Color {
    Green,
    White,
}

// fn print_line(color: Color, cols_printed: &[bool]) -> Result para uqe puedas procesar los .flush
// y propagarlos{
fn print_line(color: Color, cols_printed: &[bool]) {
    let mut stdout = stdout();

    for &c in cols_printed {
        if c {
            let c = thread_rng().gen_range(65..=90) as u8;
            if color == Color::White {
                print!("\x1b[38;5;15m{}", c as char);
            } else {
                print!("\x1b[38;5;40m{}", c as char);
            }
            stdout.flush();
        } else {
            print!("{}", ' ');
            stdout.flush();
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let n_col = size().unwrap().0 as usize;
    let n_row = size().unwrap().1 as usize;
    let mut cols_printed = vec![false; n_col];
    let mut stdout = stdout();
    let mut previous_row = 0;
    let mut rng = thread_rng();

    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;

    for t in &mut cols_printed {
        *t = rng.gen_bool(0.5);
    }

    for r in 1..=n_row {
        execute!(stdout, MoveTo(0, r as u16))?;
        print_line(Color::Green, &cols_printed);

        if previous_row != 0 {
            execute!(stdout, MoveTo(0, previous_row))?;
            print_line(Color::White, &cols_printed);
        }

        if previous_row > 1 {
            execute!(stdout, MoveTo(0, previous_row - 2))?;
            execute!(stdout, Clear(ClearType::CurrentLine))?;
        }

        if poll(Duration::from_millis(50))?
            && let Event::Key(event) = read()?
            && event.code == KeyCode::Char('q')
        {
            return Ok(());
        }

        sleep(Duration::from_millis(500));
        previous_row = r as u16;
    }

    sleep(Duration::from_millis(50));
    disable_raw_mode()
}
