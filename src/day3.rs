use std::fs;

pub fn run() {
    println!("Running day3.rs");
    let filename = "src/day3.txt";
    let lines = fs::read_to_string(filename).unwrap();
    let vec = lines
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let fst = first(&vec);
    let snd = second(&vec);

    println!("Result first: {}", fst);
    println!("Result second: {}", snd);
}

fn first(matrix: &Vec<Vec<u32>>) -> isize {
    let transposed_matrix = transpose(matrix);
    let fst_int = isize::from_str_radix(get_binary(&transposed_matrix, false).as_str(), 2).unwrap();
    let snd_int = isize::from_str_radix(get_binary(&transposed_matrix, true).as_str(), 2).unwrap();
    return fst_int * snd_int;
}

fn second(matrix: &Vec<Vec<u32>>) -> isize {
    let fst = isize::from_str_radix(reduce_matrix(&matrix, false).as_str(), 2).unwrap();
    let snd = isize::from_str_radix(reduce_matrix(&matrix, true).as_str(), 2).unwrap();
    return fst * snd;
}

fn reduce_matrix(matrix: &Vec<Vec<u32>>, get_greater_number: bool) -> String {
    let mut position = 0;
    let mut next_matrix: Vec<Vec<u32>> = matrix.clone();

    loop {
        if next_matrix.len() == 1 {
            return next_matrix
                .first()
                .unwrap()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>();
        }

        let row = &transpose(&next_matrix)[position];
        let count_0 = row.into_iter().filter(|&x| *x == 0).count();
        let count_1 = row.into_iter().filter(|&x| *x == 1).count();
        let keep_with = get_0_or_1(count_0, count_1, get_greater_number);
        next_matrix = next_matrix
            .into_iter()
            .filter(|x| x[position] == keep_with)
            .collect::<Vec<Vec<u32>>>();
        position += 1;
    }
}

fn transpose(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut transposed_matrix: Vec<Vec<u32>> = vec![];
    for row in matrix.into_iter() {
        for (j, x) in row.into_iter().enumerate() {
            if let Some(inner_vec) = transposed_matrix.get_mut(j) {
                inner_vec.push(*x);
            } else {
                transposed_matrix.push(vec![*x])
            }
        }
    }
    return transposed_matrix;
}

fn get_binary(matrix: &Vec<Vec<u32>>, get_greater_number: bool) -> String {
    let mut output = String::new();
    for row in matrix {
        let count_0 = row.into_iter().filter(|&x| *x == 0).count();
        let count_1 = row.into_iter().filter(|&x| *x == 1).count();
        output += get_0_or_1(count_0, count_1, get_greater_number)
            .to_string()
            .as_str();
    }
    return output;
}

fn get_0_or_1(count_0: usize, count_1: usize, get_greater_number: bool) -> u32 {
    if get_greater_number {
        match count_0 > 0 && (count_0 > count_1 && count_0 != count_1) {
            true => 0,
            false => 1,
        }
    } else {
        match count_0 > 0 && (count_0 <= count_1 || count_0 == count_1) {
            true => 0,
            false => 1,
        }
    }
}

#[cfg(test)]
mod test_transpose {
    use crate::day3;

    #[test]
    fn transposes_matrix() {
        let matrix = vec![vec![0, 0, 0, 0], vec![1, 1, 1, 1]];
        assert_eq!(
            day3::transpose(&matrix),
            vec![vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1]]
        )
    }

    #[test]
    fn gets_binary() {
        let matrix = vec![vec![0, 1, 1], vec![1, 1, 1], vec![0, 1, 1], vec![0, 1, 1]];
        assert_eq!(day3::get_binary(&matrix, false), "1111");
        assert_eq!(day3::get_binary(&matrix, true), "0100");
    }
}
