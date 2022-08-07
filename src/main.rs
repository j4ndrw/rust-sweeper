mod field;
mod sweeper;
mod tile;

use sweeper::{Difficulty, Sweeper};

use clap::Parser;

use core::time;
use std::io::stdout;
use std::thread;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    difficulty: u8,
}

fn main() {
    let args = Args::parse();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    writeln!(stdout, "{}", termion::clear::All).unwrap();

    let mut sweeper = Sweeper::new(match args.difficulty {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        _ => Difficulty::Hard,
    });

    let mut cursor = (0, 0);
    loop {
        sweeper.display_field(&mut stdout);

        let input = stdin.next();

        if let Some(Ok(key)) = input {
            let (should_exit, updated_cursor) = sweeper.tick(&key, cursor);
            if should_exit {
                break;
            }

            cursor = updated_cursor;
        }

        stdout.lock().flush().unwrap();

        thread::sleep(time::Duration::from_millis(50));
    }
}
