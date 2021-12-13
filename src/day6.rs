use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct LanternFish {
    timer: i32,
}

impl LanternFish {
    pub fn spawn() -> LanternFish {
        LanternFish { timer: 8 }
    }
}

impl FromStr for LanternFish {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timer = s.parse::<i32>().unwrap();
        Ok(LanternFish { timer })
    }
}

impl Iterator for LanternFish {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.timer == 0 {
            self.timer = 6;
            return None;
        } else {
            self.timer -= 1;
        }
        Some(self.timer)
    }
}

static DAYS: i32 = 256;

pub fn run() {
    println!("Running day6.rs");
    let mut lantern_fishes: Vec<LanternFish> = include_str!("day6.input")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut count_map: HashMap<i32, u64> = HashMap::new();
    let mut count = 0;
    for lantern_fish in lantern_fishes.iter_mut() {
        if let Some(v) = count_map.get(&lantern_fish.timer) {
            count += v;
        } else {
            let timer = lantern_fish.timer;
            lantern_fish.timer -= 1;
            let current_count = calculate(lantern_fish, 1, 1);
            count_map.insert(timer, current_count);
            count += current_count;
        }
    }

    println!("Result: {:?}", count);
}

fn calculate(lantern_fish: &mut LanternFish, current_day: i32, count: u64) -> u64 {
    let next_spawn_day = lantern_fish.timer + 1 + current_day;
    if next_spawn_day > DAYS {
        return count;
    }
    lantern_fish.next();
    return calculate(
        lantern_fish,
        next_spawn_day,
        calculate(&mut LanternFish::spawn(), next_spawn_day, count + 1),
    );
}
