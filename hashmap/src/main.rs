use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let mut scores: HashMap<String, i32> =
    //     teams.into_iter().zip(initial_scores.into_iter()).collect();

    let blue_name = String::from("Blue");
    let red_name = String::from("Red");
    scores.insert(&blue_name, 10);
    scores.insert(&red_name, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
}
