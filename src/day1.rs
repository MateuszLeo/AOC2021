use std::fs;
use std::path::Iter;

#[derive(Debug)]
struct Result {
    value: i64,
    flag: Option<bool>,
}

pub fn run() {
    println!("Running day1.rs");
    let filename = "src/day1.txt";
    let contents = fs::read_to_string(filename).unwrap();

    let vec = contents.lines().map(|x| x.parse::<i64>().unwrap());
    let v = vec.collect::<Vec<i64>>();
    let fst = first(&v);
    let snd = second(&v);

    println!(
        "Result first: {:?}",
        fst.iter().filter(|&x| { x.flag.is_some() }).count()
    );
    println!(
        "Result second: {:?}",
        snd.iter().filter(|&x| { x.flag.is_some() }).count()
    );
}

fn first(iter: &Vec<i64>) -> Vec<Result> {
    let mut previous: i64 = 0;
    let mut result_vec: Vec<Result> = vec![];
    for &value in iter.into_iter().skip(1) {
        if previous < value {
            result_vec.push(Result {
                flag: Some(true),
                value,
            })
        } else {
            result_vec.push(Result { flag: None, value })
        }
        previous = value
    }
    return result_vec;
}

fn second(vec: &Vec<i64>) -> Vec<Result> {
    let mut result_vec: Vec<&[i64]> = vec![];
    let mut start = 0;
    let mut end = 3;

    while vec.len() >= end {
        let next = &vec[start..end];
        result_vec.push(next);
        start += 1;
        end += 1;
    }

    return first(
        &result_vec
            .into_iter()
            .map(|x| x.into_iter().sum())
            .collect(),
    );
}
