use std::{collections::HashMap, hash::BuildHasher};

fn main() {
    // like vectors HashMap is homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue Team"), 10);
    scores.insert(String::from("Red Team"), 50);

    let team_name = String::from("Blue Team");
    // let error_message = format!("{} score is missing", &team_name);

    let score = scores
        .get(&team_name)
        .expect(&format!("{} score is missing", &team_name));
    println!("{} score is {}", &team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* Ownership
    - Types like i32 that implements Copy trait, the values are copied into the HashMap
    - But types like String, the values will be moved and HashMap will be the owner of the values */
    let yellow_team = String::from("Yellow Team");
    let yellow_score = 35;
    scores.insert(yellow_team, yellow_score);
    // yellow_team is invalid at this point. yellow_score should be available.
    println!("{}: {}", "yellow_team", yellow_score);

    /* Updating a HashMap */

    // overwrite
    scores.insert(String::from("Green Team"), 34);
    scores.insert(String::from("Green Team"), 68);
    println!("{:?}", scores);

    // add only if a key isn't present
    scores.entry(String::from("Green")).or_insert(100);
    scores.entry(String::from("Purple")).or_insert(20);
    println!("{:?}", scores);

    // update based on current value
    let current_score = scores.entry(String::from("Purple")).or_insert(0);
    *current_score += 1;
    println!("{:?}", scores);

    /* Hashing Functions
    - using SipHash
    - resistant to DoS attack, but not the fastest.
    - hasher / BuildHasher
    */
}
