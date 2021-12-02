use std::fs;

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

struct Position {
    x: i32,
    y: i32,
    aim: i32,
    use_aim: bool,
}

impl Position {
    fn forward(&mut self, value: i32) {
        self.x += value;
        if self.use_aim {
            self.y += self.aim * value;
        }
    }

    fn down(&mut self, value: i32) {
        if self.use_aim {
            self.aim += value
        } else {
            self.y += value;
        }
    }

    fn up(&mut self, value: i32) {
        if self.use_aim {
            self.aim -= value
        } else {
            self.y -= value;
        }
    }
}

pub fn run() {
    println!("Running day2.rs");
    let filename = "src/day2.txt";
    let lines = fs::read_to_string(filename).unwrap();
    let vec = lines
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|line| match (line[0], line[1].parse::<i32>().unwrap()) {
            ("down", value) => Command::Down(value),
            ("up", value) => Command::Up(value),
            ("forward", value) => Command::Forward(value),
            (command, _) => panic!("{} not defined", command),
        })
        .collect::<Vec<Command>>();

    let fst = calculate_position(&vec, false);
    let snd = calculate_position(&vec, true);

    println!(
        "Result first: x:{}, y:{}, position: {}",
        fst.x,
        fst.y,
        fst.x * fst.y
    );
    println!(
        "Result second: x:{}, y:{}, position: {}",
        snd.x,
        snd.y,
        snd.x * snd.y
    );
}

fn calculate_position(vec: &Vec<Command>, use_aim: bool) -> Position {
    let mut position = Position {
        x: 0,
        y: 0,
        aim: 0,
        use_aim,
    };
    for command in vec {
        match command {
            Command::Down(value) => position.down(*value),
            Command::Up(value) => position.up(*value),
            Command::Forward(value) => position.forward(*value),
        };
    }
    return position;
}
