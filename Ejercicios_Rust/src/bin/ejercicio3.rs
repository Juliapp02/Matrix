/*
*   Exercise 3:
*       1. Create dinamic vector with size = num_cols terminal
*       2. Use u8 and then convert into char.
*       3. Initialize array.
*       4. Print array in one line, move to next line, delete previous
*          one and print the array again.
*
*
*
*       Las letras en verde
*
* */

use std::{
    io::{Result, Write, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use rand::{Rng, thread_rng};

fn main() -> Result<()> {
    enable_raw_mode()?;

    sleep(Duration::from_secs(1));

    let n_col = size().unwrap().0 as usize;
    let n_row = size().unwrap().1;
    let mut v = vec![0u8; n_col];
    let mut stdout = stdout();

    execute!(stdout, Hide)?;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;

    for _ in 1..=n_row {
        for i in &mut v {
            let n = thread_rng().gen_range(65..=90);
            *i = n;

            print!("\x1b[38;5;40m{}\x1b", *i as char);
            stdout.flush()?;

            //Prints vertical lines
            //execute!(stdout, MoveLeft(1))?;
            //execute!(stdout, MoveDown(1))?;
        }

        if poll(Duration::from_millis(50))?
            && let Event::Key(event) = read()?
            && event.code == KeyCode::Char('q')
        {
            return Ok(());
        }

        sleep(Duration::from_millis(500));
        execute!(stdout, Clear(ClearType::FromCursorUp))?;

        //Moves to the next column (for printing vertical lines)
        //execute!(stdout, MoveTo(_ as u16 + 1, 0))?; //Change _
    }

    sleep(Duration::from_secs(2));
    disable_raw_mode()
}
