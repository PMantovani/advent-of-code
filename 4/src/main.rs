use std::{fs};

fn main() {
    // part_1();
    part_2();
}

#[allow(dead_code)]
fn part_1() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();

    let char_matrix = lines.map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut result: Vec<Vec<char>> = Vec::new();

    for (line, line_values) in char_matrix.iter().enumerate() {
        let mut new_line: Vec<char> = Vec::new();

        for (col, c) in line_values.iter().enumerate() {
            let result_char = match c {
                '.' => '.',
                '@' => check_surroundings(line, col, &char_matrix),
                _ => panic!("Unknown char")
            };
            new_line.push(result_char);
        }
        result.push(new_line);
    }

    let count = result.iter()
        .fold(0, |acc, x| acc + 
            x.iter().fold(0, |acc, y| acc + if *y == 'x' { 1 } else {0}));

    // for line in result {
    //     println!("{:?}", line);
    // }

    println!("{:?}", count);
}

fn part_2() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut char_matrix = lines.map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut last_count = 0;
    let mut count: u32;

    loop {
        let mut result: Vec<Vec<char>> = Vec::new();

        for (line, line_values) in char_matrix.iter().enumerate() {
            let mut new_line: Vec<char> = Vec::new();
    
            for (col, c) in line_values.iter().enumerate() {
                let result_char = match c {
                    '@' => check_surroundings(line, col, &char_matrix),
                    c => *c
                };
                new_line.push(result_char);
            }
            result.push(new_line);
        }
    
        count = result.iter()
            .fold(0, |acc, x| acc + 
                x.iter().fold(0, |acc, y| acc + if *y == 'x' { 1 } else {0}));

        if count == last_count { break; }
        last_count = count;
        char_matrix = result;
    }

    println!("{:?}", count);
}


fn check_surroundings(line: usize, col: usize, matrix: &Vec<Vec<char>>) -> char  {
    let mut surrounding_rolls: u32 = 0;

    let first_col = col == 0;
    let last_col = col == matrix[line].len() - 1;
    let first_line = line == 0;
    let last_line = line == matrix.len() - 1;
    
    if !first_col && !first_line {
        if matrix[line - 1][col - 1] == '@' { surrounding_rolls += 1}
    }
    if !first_line {
        if matrix[line - 1][col] == '@' { surrounding_rolls += 1; }
    }
    if !first_line && !last_col {
        if matrix[line - 1][col + 1] == '@' { surrounding_rolls += 1}
    }
    if !first_col {
        if matrix[line][col - 1] == '@' { surrounding_rolls += 1; }
    }
    if !last_col {
        if matrix[line][col + 1] == '@' { surrounding_rolls += 1; }
    }
    if !first_col && !last_line {
        if matrix[line + 1][col - 1] == '@' { surrounding_rolls += 1}
    }
    if !last_line {
        if matrix[line + 1][col] == '@' { surrounding_rolls += 1; }
    }
    if !last_col && !last_line {
        if matrix[line + 1][col + 1] == '@' { surrounding_rolls += 1}
    }

    if surrounding_rolls >= 4 {
        return '@';
    }
    return 'x';
}