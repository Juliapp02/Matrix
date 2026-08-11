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
    cursor::{Hide, MoveTo, MoveToNextLine, RestorePosition, SavePosition},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

use rand::{Rng, thread_rng};

enum Color {
    Green,
    White,
}

fn print_line(num_col: usize, color: Color, cols_printed: Vec<bool>) {
    let mut stdout = stdout();

    for i in 1..num_col {
        if cols_printed[i] {
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
    let mut row_count = 1;
    let mut rng = thread_rng();

    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;

    for t in &mut cols_printed {
        *t = rng.gen_bool(0.5);
    }

    for _ in 1..=n_row {
        for i in 1..n_col {
            if cols_printed[i] {
                let c = rng.gen_range(65..=90) as u8;
                print!("\x1b[38;5;40m{}", c as char);
                stdout.flush()?;
            } else {
                print!("{}", ' ');
                stdout.flush()?;
            }
        }

        if row_count == 2 {
            execute!(stdout, MoveTo(0, row_count - 2))?;
            execute!(stdout, Clear(ClearType::All));

            for i in 1..n_col {
                if cols_printed[i] {
                    let c = rng.gen_range(65..=90) as u8;
                    print!("\x1b[38;5;15m{}", c as char);
                    stdout.flush()?;
                } else {
                    print!("{}", ' ');
                    stdout.flush()?;
                }
            }

            execute!(stdout, MoveToNextLine(1))?;

            for i in 1..n_col {
                if cols_printed[i] {
                    let c = rng.gen_range(65..=90) as u8;
                    print!("\x1b[38;5;40m{}", c as char);
                    stdout.flush()?;
                } else {
                    print!("{}", ' ');
                    stdout.flush()?;
                }
            }

            //execute!(stdout, MoveTo(n_col as u16, row_count as u16))?;
        }

        if row_count >= 3 {
            execute!(stdout, MoveTo(0, row_count - 3))?;
            execute!(stdout, Clear(ClearType::All))?;
            //execute!(stdout, Clear(ClearType::FromCursorUp))?;

            for i in 1..n_col {
                if cols_printed[i] {
                    let c = rng.gen_range(65..=90) as u8;
                    print!("\x1b[38;5;15m{}", c as char);
                    stdout.flush()?;
                } else {
                    print!("{}", ' ');
                    stdout.flush()?;
                }
            }

            execute!(stdout, MoveToNextLine(1))?;

            for i in 1..n_col {
                if cols_printed[i] {
                    let c = rng.gen_range(65..=90) as u8;
                    print!("\x1b[38;5;15m{}", c as char);
                    stdout.flush()?;
                } else {
                    print!("{}", ' ');
                    stdout.flush()?;
                }
            }

            execute!(stdout, MoveToNextLine(1))?;

            for i in 1..n_col {
                if cols_printed[i] {
                    let c = rng.gen_range(65..=90) as u8;
                    print!("\x1b[38;5;40m{}", c as char);
                    stdout.flush()?;
                } else {
                    print!("{}", ' ');
                    stdout.flush()?;
                }
            }
        }

        execute!(stdout, MoveToNextLine(1))?;

        if poll(Duration::from_millis(50))?
            && let Event::Key(event) = read()?
            && event.code == KeyCode::Char('q')
        {
            return Ok(());
        }

        sleep(Duration::from_millis(500));
        row_count += 1;
    }

    sleep(Duration::from_millis(50));
    disable_raw_mode()
}
