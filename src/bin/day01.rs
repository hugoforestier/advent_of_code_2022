use std::io;

fn part1(input: &str) -> io::Result<()> {
    let parsed = input.split('\n');
    let mut totals: Vec<i32> = vec![];
    let mut tmp_total = 0;
    
    for c in parsed {
        let c = c.parse::<i32>();
        if c.is_ok() {
            tmp_total += c.unwrap();
        } else {
            totals.push(tmp_total);
            tmp_total = 0;
        }
    }
    println!("The highest calorie score is {:?}", totals.iter().max().unwrap());
    return Ok(());
}

fn part2(input: &str) -> io::Result<()> {
    let parsed = input.split('\n');
    let mut totals: Vec<i32> = vec![];
    let mut tmp_total = 0;
    
    for c in parsed {
        let c = c.parse::<i32>();
        if c.is_ok() {
            tmp_total += c.unwrap();
        } else {
            totals.push(tmp_total);
            tmp_total = 0;
        }
    }
    totals.sort_by(|a, b| b.cmp(a));
    println!("The highest calorie scores are {:?}", totals[0..3].iter().sum::<i32>());
    return Ok(());
}

fn main() -> io::Result<()> {
    let input = include_str!("test1.input");

    part1(&input)?;
    part2(&input)?;
    Ok(())
}
