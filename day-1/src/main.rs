use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::u32;

fn parse_input(buffer: &str) -> (Vec<&str>, Vec<&str>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in buffer.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        list_a.push(v[0]);
        list_b.push(v[1]);
    }

    (list_a, list_b)
}

fn total_distance(sorted_list_a: Vec<&str>, sorted_list_b: Vec<&str>) -> u32 {
    let mut result = Vec::new();

    for i in 0..sorted_list_a.len() {
        let num_a = sorted_list_a[i].parse::<u32>().unwrap();
        let num_b = sorted_list_b[i].parse::<u32>().unwrap();
        let distance = num_a.abs_diff(num_b);
        result.push(distance);
    }

    result.iter().sum()
}

fn main() -> Result<()> {
    let mut file = File::open("./day-1/input-1.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let (mut list_a, mut list_b) = parse_input(&buffer);

    list_a.sort();
    list_b.sort();

    println!("Total Distance: {}", total_distance(list_a, list_b));

    Ok(())
}
