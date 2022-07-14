use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores {:?}", scores);

    let field_name = String::from("Blue");
    let field_value = 10;
    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    map.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("Score = {:?}", score);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    map.insert(String::from("Yellow"), 432);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    map.entry(String::from("Yellow")).or_insert(123847);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
