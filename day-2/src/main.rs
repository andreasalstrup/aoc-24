use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn parse_input(buffer: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    for line in buffer.lines() {
        let v = line
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect();
        result.push(v);
    }

    result
}

fn is_safe(level: &Vec<u32>) -> bool {
    fn check(level: &Vec<u32>) -> bool {
        let offset: Vec<u32> = level
            .iter()
            .cycle()
            .skip(1)
            .take(level.len())
            .cloned()
            .collect();

        let mut pairs: Vec<(&u32, &u32)> = level.iter().zip(offset.iter()).collect();
        pairs.truncate(pairs.len() - 1);

        let check = pairs.iter().all(|&(a, b)| a != b && (a.abs_diff(*b) <= 3));
        let increasing = pairs.iter().all(|&(a, b)| a < b);
        let decreasing = pairs.iter().all(|&(a, b)| a > b);
        check && (increasing || decreasing)
    }

    if check(level) {
        true
    } else {
        for i in 0..level.len() {
            let mut dampener_vec = level.clone();
            dampener_vec.remove(i);
            if check(&dampener_vec) {
                return true;
            }
        }
        false
    }
}

fn safe_reports(levels: &Vec<Vec<u32>>) -> u32 {
    levels
        .iter()
        .fold(0, |acc, l| if is_safe(l) { acc + 1 } else { acc })
}

fn main() -> Result<()> {
    let mut file = File::open("./day-2/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let levels = parse_input(&buffer);
    println!("Safe reports: {}", safe_reports(&levels));

    Ok(())
}
