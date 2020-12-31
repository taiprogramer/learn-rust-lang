use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    match read_something_from_file() {
        Ok(s) => {
            println!("============== BEGIN CONTENT ==============");
            println!("{}", s);
        },
        Err(err) => {
            println!("============== BEGIN ERROR ==============");
            println!("{:?}", err);
            panic!("Error happened.");
        }
    }
}

fn read_something_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

