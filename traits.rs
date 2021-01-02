struct Cat { name: String }

trait Animal {
    fn sound(&self) -> String;
}

impl Animal for Cat {
    fn sound(&self) -> String {
        "Meow".to_owned()
    }
}

fn main() {
    let a = Cat { name: "Caty".to_owned() };

    println!("{}: {}", a.name, a.sound());
}

