use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Red")).or_insert(50); // if the key doesn't exist, insert the value

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0); // by calling copied to get an Option<i32> rather than an Option<&i32>

    println!("Score for team {team_name}: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //-------------------------------------------
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // dereference the count to get to the i32 value and add 1 to it
    }

    println!("{map:?}"); //{"hello": 1, "world": 2, "wonderful": 1}
}
