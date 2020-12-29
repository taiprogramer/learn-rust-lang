#[derive(Debug)]
enum Gender {
    MALE,
    FEMALE
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    sex: Gender
}

fn main() {
    let a = User {
        username: String::from("user"),
        password: String::from("pass"),
        sex: Gender::MALE
    };
    let b = User {
        username: String::from("user"),
        password: String::from("pass"),
        sex: Gender::FEMALE
    };

    accept_male(&a); // Accepted
    accept_male(&b); // Rejected

    let c: Option<i8> = Some(1);
    let d: Option<i8> = None;
    println!("c = {}", extract_num(c));
    println!("d = {}", extract_num(d));

    if let Some(1) = c {
        println!("c is {:?}", c);
    }
}

fn accept_male(user: &User) {
    match user.sex {
        Gender::MALE => println!("Accepted"),
        _ => println!("Rejected")
    }
}

fn extract_num(a: Option<i8>) -> i8 {
    match a {
        Some(i) => i,
        None => 0
    }
}

