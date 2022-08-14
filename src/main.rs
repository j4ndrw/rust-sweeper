mod field;
mod sweeper;
mod tile;

use sweeper::{Difficulty, Position, Sweeper};

use clap::Parser;

use std::io::stdout;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    difficulty: Option<u8>,

    #[clap(short, long, value_parser)]
    rows: Option<usize>,

    #[clap(short, long, value_parser)]
    cols: Option<usize>,

    #[clap(short, long, value_parser)]
    bomb_percentile: Option<f32>,
}

fn main() {
    let args = Args::parse();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let difficulty = match args.difficulty {
        Some(0) => Difficulty::Easy,
        Some(1) => Difficulty::Medium,
        Some(2) => Difficulty::Hard,
        Some(3) => Difficulty::Nightmare,
        _ => Difficulty::Custom,
    };
    let custom_params = match (args.rows, args.cols, args.bomb_percentile) {
        (Some(rows), Some(cols), Some(bomb_percentile)) => Some((rows, cols, bomb_percentile)),
        _ => None,
    };
    let mut sweeper = Sweeper::new(difficulty, custom_params);
    let mut cursor = Position(sweeper.field.rows / 2, sweeper.field.cols / 2);

    writeln!(stdout, "{}", termion::clear::All).unwrap();

    sweeper.select(&cursor);
    sweeper.display_field(&mut stdout);

    loop {
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            let (should_exit, should_restart, updated_cursor) = sweeper.tick(&key, cursor);
            if should_exit {
                break;
            }

            if should_restart {
                sweeper = Sweeper::new(difficulty, custom_params);
            }

            cursor = updated_cursor;
            sweeper.display_field(&mut stdout);
        }

        stdout.lock().flush().unwrap();
    }
}
