fn main() {
    let mut s = String::from("Hello, world!");
    s = move_ownership(s);
    println!("{}", s);

    let s1 = String::from("Reference and move_ownership!");
    reference(&s1);
    println!("{}", s1);

    let mut s2 = String::from("Mutable Reference!");
    mut_reference(&mut s2);
    println!("{}", s2);
}

fn move_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn reference(s: &String) {
    println!("{}", s);
}

fn mut_reference(s: &mut String) {
    s.insert_str(0, "pushed string!");
    *s = String::from("Replacement!");
}
