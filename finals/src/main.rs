use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        println!("Hello threads!");
    });
    println!("Hello main!");

    handle.join().unwrap();
}
