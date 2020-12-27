#[derive(Debug)]
struct User {
    username: String,
    password: String
}

impl User {
    // Associated function: doesn't take &self parameter
    fn new_user(username: &str, password: &str) -> User {
        User {
            username: String::from(username),
            password: String::from(password)
        }
    }
    // Method: take &self parameter
    fn get_pass(&self) -> &str {
        &self.password
    }
}

// tuple struct
struct RGB(u8, u8, u8);

fn main() {
    let a = User {
        username: String::from("user"),
        password: String::from("pass")
    };
    println!("{:?}", a); // Print in Debug mode
    println!("{:#?}", a); // Print in Debug mode but more readable

    let b = User::new_user("user", "pass");
    println!("password of user user is: {}", b.get_pass());

    let c = RGB(255, 0, 255);
    println!("R: {}, G: {}, B: {}", c.0, c.1, c.2);
}

