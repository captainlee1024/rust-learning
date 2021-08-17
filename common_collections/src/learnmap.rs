use std::collections::HashMap;

#[test]
fn hashmap_one() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

#[test]
fn create_map_from_vec() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(team_score) => println!("team score: {}", team_score),
        None => println!("not found: Blue"),
    }
}

#[test]
fn map_wonership() {
    let _field_name = String::from("Favorite color");
    let _field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_name);
}

#[test]
fn get_item_from_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Green"), 10);
    scores.insert(String::from("Pink"), 10);

    let team_name = String::from("Pink");
    let score = scores.get(&team_name).unwrap();
    println!("Pink score: {}", score);

    for (key, value) in &scores {
        println!("team {}: score {}", key, value);
    }
}

#[test]
fn update_with_key() {
    let mut scores = HashMap::new();
    // over write
    scores.insert(String::from("Orange"), 10);
    println!("{:?}", scores);

    scores.insert(String::from("Orange"), 20);
    println!("{:?}", scores);

    // 只在键不存在时插入
    scores.entry(String::from("Orange")).or_insert(50);
    scores.entry(String::from("Cyan")).or_insert(50);
    println!("{:?}", scores);

    //根据旧值更新一个值
    let text = "Green Orange Orange Cyan Cyan Cyan Blue";
    for color in text.split_whitespace() {
        let count = scores.entry(String::from(color)).or_insert(0);
        *count += 10;
    }

    println!("{:?}", scores);
}
