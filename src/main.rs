mod field;
mod tile;
mod sweeper;

use field::Field;
use sweeper::{Difficulty, Sweeper};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser)]
   difficulty: u8
}


fn main() {
    let args = Args::parse();
    let mut sweeper = Sweeper::new(match args.difficulty {
        0 => Difficulty::Easy,
        1 => Difficulty::Medium,
        _ => Difficulty::Hard,
    });

    sweeper.field.populate((5, 5));

    println!("{}", sweeper.field);
}
