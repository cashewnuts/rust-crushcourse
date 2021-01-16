fn main() {
    println!("Hello, world!");

    let (bunnies, _carrots) = (8, 50);
    println!("{}", bunnies);

    let mut bunnies = 16;
    println!("{}", bunnies);
    bunnies = 2;
    println!("{}", bunnies);

    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    }
    // println!("{}, {}", x, y);
    {
        let x = 99;
        println!("{}", x);
    }
    println!("{}", x);
}
