use std::fs::File;

fn main() {
    let x: Option<i32> = None;
    // x = Some(5);
    // x.is_some();
    // x.is_none();
    for i in x {
        println!("{}", i);
    }

    if let Some(y) = x {
        println!("Some is: {}", y);
    } else {
        println!("None");
    }

    let res = File::open("foo");
    match res {
        Ok(f) => {
            println!("File struct: {:?}", f)
        },
        Err(e) => {
            println!("Error struct: {:?}", e);
        }
    }
}
