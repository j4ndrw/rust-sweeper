mod field;
mod tile;

use field::Field;

fn main() {
    let f = Field::create(10, 10);
    println!("{}", f);
}
