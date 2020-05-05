use rollenspielsache::dice::Roll;
use std::str::FromStr;

fn main() {
    let roll = Roll::from_str("2d20+7").unwrap();
    println!("Rolling {}...\n{}", roll, roll.execute().total());
}
