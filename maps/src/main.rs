use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores.get("Blue"));
    println!("{:?}", scores.get("Yellow"));
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");
    scores.entry(team_name).or_insert(30);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    let text = "hello world wonderful world";

    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{word_counts:?}");

}
