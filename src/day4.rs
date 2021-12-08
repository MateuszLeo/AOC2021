use std::fs;

#[derive(Debug)]
struct MyCell {
    value: u32,
    flag: bool,
}

impl MyCell {
    pub fn new(value: u32) -> MyCell {
        MyCell { value, flag: false }
    }

    pub fn mark(&mut self, value: u32) {
        if value == self.value {
            self.flag = true;
        }
    }
}

#[derive(Debug)]
struct Found {
    when_n: u32,
    board_index: usize,
}

pub fn run() {
    println!("Running day4.rs");
    let filename = "src/day4.input";
    let lines = fs::read_to_string(filename).unwrap();
    let vec = lines
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let random_numbers = &vec[0]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let parsed_boards: Vec<Vec<u32>> = (&vec[1..])
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<u32>())
                .filter(Result::is_ok)
                .map(Result::ok)
                .map(Option::unwrap)
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut cell_boards: Vec<Vec<MyCell>> = parsed_boards
        .into_iter()
        .map(|row| row.into_iter().map(MyCell::new).collect::<Vec<MyCell>>())
        .collect();

    let mut boards = cell_boards
        .chunks_mut(5)
        .collect::<Vec<&mut [Vec<MyCell>]>>();

    let found_boards = play(&mut boards, random_numbers);

    let first_found = found_boards.get(0).unwrap();
    let first_board = boards.get(first_found.board_index).unwrap();
    let fist_board_sum = sum(&first_board);
    println!("Result first: {:?}", fist_board_sum * first_found.when_n);

    let last_found = found_boards.last().unwrap();
    let last_board = boards.get(last_found.board_index).unwrap();
    let last_board_sum = sum(&last_board);
    println!("Result second: {:?}", last_board_sum * last_found.when_n);
}

fn sum(board: &[Vec<MyCell>]) -> u32 {
    board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|cell| cell.flag == false)
                .map(|cell| cell.value)
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn transpose(matrix: &[Vec<MyCell>]) -> Vec<Vec<MyCell>> {
    let mut transposed_matrix: Vec<Vec<MyCell>> = vec![];
    for row in matrix.into_iter() {
        for (column_index, x) in row.into_iter().enumerate() {
            if let Some(inner_vec) = transposed_matrix.get_mut(column_index) {
                inner_vec.push(MyCell {
                    flag: x.flag,
                    value: x.value,
                });
            } else {
                transposed_matrix.push(vec![MyCell {
                    flag: x.flag,
                    value: x.value,
                }])
            }
        }
    }
    return transposed_matrix;
}

fn has_won(board: &[Vec<MyCell>]) -> bool {
    for row in board.iter() {
        let count = row.into_iter().filter(|cell| cell.flag).count();
        if count == row.len() {
            return true;
        }
    }
    return false;
}

fn play(boards: &mut Vec<&mut [Vec<MyCell>]>, numbers: &Vec<u32>) -> Vec<Found> {
    let board_count = boards.len();
    let mut vec: Vec<Found> = vec![];
    for n in numbers {
        for board_index in 0..board_count {
            if let Some(_) = vec.iter().find(|found| found.board_index == board_index) {
                continue;
            }
            for x in 0..5 {
                for y in 0..5 {
                    boards[board_index][x][y].mark(*n);
                }
            }
            if has_won(&boards[board_index]) || has_won(&transpose(&boards[board_index])) {
                vec.push(Found {
                    board_index,
                    when_n: *n,
                })
            }
        }
    }
    return vec;
}
