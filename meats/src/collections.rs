use std::collections::HashMap;

fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(2);
    // v.push(4);
    // v.push(6);
    // let _x = v.pop();

    let mut v = vec![2,4,6];
    let _x = v.pop();
    println!("{}", v[1]);

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();

    println!("{:?}, five: {}", h, have_five);
}
