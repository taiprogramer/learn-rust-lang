fn main() {
    let a = String::from("Hello");
    let b = String::from("World");
    let c: String = format!("{}, {}.", a, b);
    let d = a + &b[..];

    println!("{}\n{}\n{}", b, c, d);
}

