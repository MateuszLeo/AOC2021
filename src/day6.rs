use std::str::FromStr;

#[derive(Debug)]
struct LanternFish {
    timer: u32,
}

impl LanternFish {
    pub fn spawn() -> LanternFish {
        LanternFish { timer: 8 }
    }
}

impl FromStr for LanternFish {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timer = s.parse::<u32>().unwrap();
        Ok((LanternFish { timer }))
    }
}

impl Iterator for LanternFish {
    type Item = u32;
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

pub fn run() {
    println!("Running day6.rs");
    let mut lantern_fishes: Vec<LanternFish> = include_str!("day6.test")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let days = 256;
    for _ in 0..days {
        let mut spawned: Vec<LanternFish> = vec![];
        for lantern_fish in lantern_fishes.iter_mut() {
            if let None = lantern_fish.next() {
                spawned.push(LanternFish::spawn());
            }
        }
        lantern_fishes.append(&mut spawned);
    }

    println!("Result: {:?}", lantern_fishes.len());
    // println!("Result: {:?}", initial);
}
/*
3,4,3,1,2
*/
