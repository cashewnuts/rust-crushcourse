fn main() {
    // let fox = RedFox {
    //     enemy: true,
    //     life: 80,
    // };
    let fox = RedFox::new();
    println!("fox life: {}, is enemy: {}", fox.life, fox.enemy);

    println!("Noisy fox: {}", fox.get_noise());
    print_noise(fox);

    let robot = Roboto {};
    robot.run();
}

struct RedFox {
    enemy: bool,
    life: u32,
}

impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow!!"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}

struct Roboto {}
impl Run for Roboto {}
