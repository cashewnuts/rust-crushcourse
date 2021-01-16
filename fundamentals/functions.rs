fn main() {
    let x = do_stuff(2.0, 12.5);
    println!("{}", x);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    println!("{} {}-oz sarsaparilla(s)!", qty, oz);
    qty * oz
}
