use rand::Rng;
use fundamentals::greet;

fn main() {
    greet();
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen_range(0, 10);
    println!("{}", x);
}
