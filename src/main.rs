use rollenspielsache::dice::Roll;

fn main() {
    let roll = Roll::default();
    println!("Rolling {}...\n{}", roll, roll.execute().total());
}
