use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::Duration,
    usize,
};

use crossterm::{
    cursor::{Hide, MoveTo},
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

fn print_line(color: Color, cols_printed: &[(bool, usize)], n_row: usize) -> Result<()> {
    let mut stdout = stdout();

    for &c in cols_printed {
        if c.0 && c.1 > n_row {
            let c = thread_rng().gen_range(33..=126) as u8;
            if color == Color::White {
                print!("\x1b[38;5;15m{}", c as char);
            } else {
                print!("\x1b[38;5;40m{}", c as char);
            }
            stdout.flush()?;
        } else {
            print!(" ");
            stdout.flush()?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let n_col = size().unwrap().0 as usize;
    let n_row = size().unwrap().1 as usize;
    let mut cols_printed = vec![(false, 0); n_col];
    let mut stdout = stdout();
    //let mut previous_row = 0;
    let mut rng = thread_rng();
    let mut stop = Err(());

    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;

    /*for t in &mut cols_printed {
        *t = (rng.gen_bool(0.5), rng.gen_range(0..=n_row));
    }*/
    while stop != Ok(()) {
        for t in &mut cols_printed {
            *t = (rng.gen_bool(0.5), rng.gen_range(0..=n_row));
        }

        let mut previous_row = 0;
        for r in 1..=n_row {
            /*if previous_row == 0 {
                execute!(stdout, Clear(ClearType::All))?;
            }*/
            execute!(stdout, MoveTo(0, r as u16))?;
            print_line(Color::White, &cols_printed, r)?;

            if previous_row != 0 {
                execute!(stdout, MoveTo(0, r as u16 - 1))?;
                print_line(Color::Green, &cols_printed, r)?;
            }

            if poll(Duration::from_millis(50))?
                && let Event::Key(event) = read()?
                && event.code == KeyCode::Char('q')
            {
                stop = Ok(());
            }

            sleep(Duration::from_millis(50));
            previous_row = r as u16;
        }
    }

    sleep(Duration::from_millis(50));
    disable_raw_mode()
}
