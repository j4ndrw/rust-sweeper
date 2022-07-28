mod field;
mod tile;

use field::Field;

fn main() {
    let f = Field::create(20, 20);
    println!("{}", f);
}
