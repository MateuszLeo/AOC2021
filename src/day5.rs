use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::format;
use std::fs;

struct FissureMap {
    table: HashMap<String, u32>,
}

impl FissureMap {
    pub fn mark(&mut self, x: i32, y: i32) {
        let key = format!("{}-{}", x, y);
        let value = match self.table.get_mut(&key) {
            Some(value) => *value + 1,
            None => 1,
        };
        self.table.insert(key, value);
    }

    pub fn count(&self) -> usize {
        return self.table.iter().filter(|(k, &v)| v > 1).count();
    }
}

pub fn run() {
    println!("Running day5.rs");
    let filename = "src/day5.test";
    let lines = fs::read_to_string(filename).unwrap();
    let segment = lines
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|x| {
                    x.split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

    let mut fissure: FissureMap = FissureMap {
        table: HashMap::new(),
    };

    for row in segment {
        let start_position = &row[0];
        let end_position = &row[1];
        let x1 = start_position[0];
        let y1 = start_position[1];
        let x2 = end_position[0];
        let y2 = end_position[1];

        let is_collinear = ((x1 - x2) as f32).abs() == ((y1 - y2) as f32).abs();

        if is_collinear {
            let x_factor = if x1 < x2 { 1 } else { -1 };
            let y_factor = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            let mut should_stop = false;
            while !should_stop {
                should_stop = if x_factor > 0 { x >= x2 } else { x <= x2 };
                fissure.mark(x, y);
                x += x_factor;
                y += y_factor;
            }
        }

        let is_straight_line = x1 == x2 || y1 == y2;
        if is_straight_line {
            let (x_start, x_end) = (min(x1, x2), max(x1, x2));
            let (y_start, y_end) = (min(y1, y2), max(y1, y2));
            (x_start..=x_end).for_each(|x| (y_start..=y_end).for_each(|y| fissure.mark(x, y)));
        };
    }

    println!("Result: {:?}", fissure.count());
}
