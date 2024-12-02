use std::collections::HashMap;
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

fn total_distance(sorted_list_a: &Vec<&str>, sorted_list_b: &Vec<&str>) -> u32 {
    let mut result = Vec::new();

    for i in 0..sorted_list_a.len() {
        let num_a = sorted_list_a[i].parse::<u32>().unwrap();
        let num_b = sorted_list_b[i].parse::<u32>().unwrap();
        let distance = num_a.abs_diff(num_b);
        result.push(distance);
    }

    result.iter().sum()
}

fn similarity_score(list_a: &Vec<&str>, list_b: &Vec<&str>) -> u32 {
    let mut result = 0;
    let mut similarity_map: HashMap<&str, u32> = HashMap::new();
    let mut iter_a = list_a.into_iter();
    let mut iter_b = list_b.into_iter().peekable();

    // Fast AF
    while let Some(a) = iter_a.next() {
        while let Some(&b) = iter_b.peek() {
            if a > b {
                iter_b.next();
            } else if a == b {
                similarity_map.entry(a).and_modify(|v| *v += 1).or_insert(1);
                iter_b.next();
            } else {
                break;
            }
        }
    }

    // Slow AF
    // for a in list_a {
    //     for b in list_b {
    //         if a == b {
    //             similarity_map.entry(a).and_modify(|v| *v += 1).or_insert(1);
    //         } else {
    //             similarity_map.entry(a).or_insert(0);
    //         }
    //     }
    // }

    for key in similarity_map.keys() {
        result += key.parse::<u32>().unwrap() * similarity_map[key];
    }

    result
}

fn main() -> Result<()> {
    let mut file = File::open("./day-1/input-1.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let (mut list_a, mut list_b) = parse_input(&buffer);

    list_a.sort();
    list_b.sort();

    println!("Total Distance: {}", total_distance(&list_a, &list_b));
    println!("Similarity_score: {}", similarity_score(&list_a, &list_b));

    Ok(())
}
