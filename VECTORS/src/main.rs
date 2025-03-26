use std::collections::HashMap;

fn main() {
    //create a HashMap
    let mut courses: HashMap<String, u16> = HashMap::new();
    courses.insert("Rust".to_string(), 90);
    courses.insert("Elixir".to_string(), 80);
    courses.insert("Haskell".to_string(), 92);
    println!("{:?}", courses);
    println!("{:?}", courses.get(&"Elixir".to_string()));

    //accesing ([])
    println!("{:?}", courses[&"Elixir".to_string()]);

    //contains_key
    if courses.contains_key(&"Elixxir".to_string()) {
        println!("contaions");
    }

    for (k, v) in courses.iter() {
        println!("{}=>{}", k, v);
    }

    //overwrite
    courses.insert("Elixir".to_string(), 0);
    println!("{:?}", courses);

    //or_insert entry
    courses.entry("Elixir Advanced".to_string()).or_insert(100);
    println!("{:?}", courses);
}
