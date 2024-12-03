use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn parse_input(buffer: &str) -> Vec<(&str, Vec<u32>)> {
    let mut mul_pair: Vec<(&str, Vec<u32>)> = Vec::new();

    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)").unwrap();
    let matches_mul: Vec<&str> = re_mul
        .find_iter(buffer.as_ref())
        .map(|m| m.as_str())
        .collect();

    let re_num = Regex::new(r"\d+").unwrap();
    for mul in matches_mul {
        let matches_num: Vec<u32> = re_num
            .find_iter(mul)
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect();
        mul_pair.push((mul, matches_num));
    }

    let mut result: Vec<(&str, Vec<u32>)> = Vec::new();
    let mut skip = false;
    for (k, v) in mul_pair.iter() {
        skip = match *k {
            "don't()" => true,
            "do()" => false,
            _ => skip,
        };

        if !skip {
            result.push((k, v.to_vec()))
        }
    }

    result
}

fn mul_corrupted_memory(mul_map: Vec<(&str, Vec<u32>)>) -> u32 {
    let mut result = 0;
    for (_, v) in mul_map {
        if !v.is_empty() {
            let p: u32 = v.into_iter().product();
            result += p;
        }
    }
    result
}

fn main() -> Result<()> {
    let mut file = File::open("./day-3/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let input = parse_input(&buffer);
    println!("Result of multiplications: {}", mul_corrupted_memory(input));

    Ok(())
}
