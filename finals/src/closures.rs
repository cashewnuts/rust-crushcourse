fn main() {
    let add = |x, y| { x + y };
    let x = add(1, 3);
    println!("{}", x);

    let s = "🍓".to_string();
    // let f = move || {
    let f = || {
        println!("{}", s);
    };

    f();
    println!("{}", s);

    let v = vec![2,4,6];
    let acc = v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);
    
    println!("Accumulator {}", acc);
}
