use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("blue"), 20);

    let team_name = "blue";

    let team_score = scores.get(team_name).copied().unwrap_or(0);

    println!("{team_score}");

    for (key, value) in scores {
        println!("{key}, {value}");
    }

    let text = "hello world wonderful world i love this world so sometimes but others i despise it";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // returns a mutable reference to the value if the value exists
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }
    println!("{map:?}");
}
