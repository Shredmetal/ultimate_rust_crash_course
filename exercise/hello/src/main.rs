use hello::greet;
use rand::{thread_rng, Rng};

fn main() {
    greet();
    let x = thread_rng().gen_range(0, 100);
    println!("RNGesus says: {}", x);
}
