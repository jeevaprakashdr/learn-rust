use std::collections::HashMap;

pub fn demo()  {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

     let mut scores: HashMap<String, i32> = HashMap::new();

     scores.insert(blue, 10);
     scores.insert(yellow, 30);

    let mut findItem = scores.get(&String::from("blue"));
    match scores.get("blue") {
        Some(_) => println!("found"),
        None => println!("none")
    };

    println!("{:?}", scores);
}