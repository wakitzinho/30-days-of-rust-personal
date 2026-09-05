use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);

    println!("numbers: {:?}", numbers);

    if let Some(last) = numbers.pop() {
        println!("number popped: {}", last)
    }

    println!("numbers after pop: {:?}", numbers);

    // hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("minho"), 50);
    scores.insert(String::from("zinho"), 60);
    scores.insert(String::from("sinho"), 70);

    println!("scores: {:?}", scores);

    let minho_score = scores.get("minho").unwrap();
    println!("minho's score: {:?}", minho_score);

}