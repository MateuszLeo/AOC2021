use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Crab {
    x: i32,
}

impl FromStr for Crab {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<i32>().unwrap();
        Ok(Crab { x })
    }
}

pub fn run() {
    println!("Running day7.rs");
    let mut crabs: Vec<Crab> = include_str!("day7.input")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Result first: {:?}", first(&crabs));
    println!("Result second: {:?}", second(&crabs));
}

fn second(crabs: &Vec<Crab>) -> i32 {
    let sum = crabs.iter().map(|c| c.x as f32).sum::<f32>();
    let median = (sum as f32 / crabs.iter().count() as f32).floor() as i32;
    return crabs
        .iter()
        .map(|crab| (crab.x - median).abs())
        .map(|steps| (1..=steps).sum::<i32>())
        .sum::<i32>();
}

fn first(crabs: &Vec<Crab>) -> i32 {
    let mut counts = vec![];
    for (i, current_crab) in crabs.iter().enumerate() {
        let mut count = crabs
            .iter()
            .take(i)
            .map(|c| (c.x - current_crab.x).abs())
            .sum::<i32>();
        for next_crab in &crabs[i + 1..] {
            count += (next_crab.x - current_crab.x).abs();
        }
        counts.push(count);
    }
    return *counts.iter().min().unwrap();
}
