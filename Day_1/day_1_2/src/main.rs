use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/test.input").expect("No such file");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines()
        .map(|l| l.expect("Error while reading lines"))
        .collect();
    let mut best_scores = [0; 3];
    let mut tmp_score = 0;
    let mut all_calories: Vec<i32> = Vec::new();

    for values in input {
        if !values.is_empty() {
            tmp_score += values.parse::<i32>().unwrap();
        } else {
            all_calories.push(tmp_score);
            tmp_score = 0;
        }
    }

    for mut calories in all_calories {
        for highest_score in best_scores.iter_mut() {
            if calories > *highest_score {
                std::mem::swap(highest_score, &mut calories);
            }
        }
    }
    println!("The best calorie scores are {best_scores:?}");
    println!("Those Elves are carrying in total {} calories!", best_scores.iter().sum::<i32>());
    Ok(())
}
