#[derive(Debug)]
struct User {
    username: String,
    password: String
}

impl User {
    fn new_user(username: &str, password: &str) -> User {
        User {
            username: String::from(username),
            password: String::from(password)
        }
    }
    
    fn get_pass(&self) -> &str {
        &self.password
    }
}

fn main() {
    let a = User {
        username: String::from("user"),
        password: String::from("pass")
    };
    println!("{:?}", a);
    println!("{:#?}", a);
    let b = User::new_user("user", "pass");
    println!("password of user user is: {}", b.get_pass());
}
