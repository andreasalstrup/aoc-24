use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use regex::Regex;

fn parse_input(buffer: &str) -> Vec<(&str, Vec<u32>)> {
    let mut result: Vec<(&str, Vec<u32>)> = Vec::new();

    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
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
        result.push((mul, matches_num));
    }

    result
}

fn mul_corrupted_memory(mul_map: Vec<(&str, Vec<u32>)>) -> u32 {
    let mut result = 0;
    for mul in mul_map {
        let p: u32 = mul.1.into_iter().product();
        result += p;
    }
    result
}

fn main() -> Result<()> {
    let mut file = File::open("./day-3/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let corrupted_mem =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

    // let input = parse_input(&corrupted_mem);

    let input = parse_input(&buffer);

    // println!("{:?}", input);

    println!(
        "Result of multiplications: {}",
        mul_corrupted_memory(input)
    );
    // 175015740

    Ok(())
}
