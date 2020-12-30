use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();
    // update map that override old value
    a.insert(String::from("word"), String::from("description"));
    // update map that respect old value
    let b = a.entry(String::from("word"))
        .or_insert(String::from("Another description"));
    
    *b = String::from("Another description");
   
    println!("{:?}", a);
}

