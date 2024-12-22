use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("blue"), 20);

    let team_name = "blue";

    let team_score = scores.get(team_name).copied().unwrap_or(0);

    println!("{team_score}");
}
