use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/test.input").expect("No such file");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines()
        .map(|l| l.expect("Error while reading lines"))
        .collect();
    let mut best_score = vec![0, 0];
    let mut tmp_score = 0;
    let mut index_current_player = 0;

    for s in &input {
        if !s.is_empty() {
            tmp_score += s.parse::<i32>().unwrap();
        } else {
            if tmp_score > best_score[0] {
                best_score[0] = tmp_score;
                best_score[1] = index_current_player;
            }
            tmp_score = 0;
            index_current_player+=1;
        }
    }
    println!("The best calorie score is {} and is by elf number {}!", best_score[0], best_score[1]);
    Ok(())
}
