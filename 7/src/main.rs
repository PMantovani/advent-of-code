use std::{collections::HashMap, fs};

fn main() {
    // part_1();
    part_2();
}

#[allow(dead_code)]
fn part_1() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines: Vec<_> = contents.lines().collect(); // collect lines to avoid moving iterator
    let mut result: Vec<Vec<u8>> = lines.iter().map(|x| x.as_bytes().to_vec()).collect();
    let line_len = lines[0].len();


    result.clone().iter().enumerate().for_each(|(i, line) | {
        if i == 0 { return }

        line.iter().enumerate().for_each(|(j, c)| {
            if *c == b'^' { return }
            if i > 0 && result[i - 1][j] == b'S' { result[i][j] = b'|' }
            if i > 0 && result[i - 1][j] == b'|' { result[i][j] = b'|' }
            if j < (line_len - 1) && result[i][j + 1] == b'^' && result[i-1][j+1] == b'|' { result[i][j] = b'|' }
            if j > 0 && result[i][j - 1] == b'^' && result[i-1][j-1] == b'|' { result[i][j] = b'|' }
        });
    });

    let mut split_count = 0;

    result.iter().enumerate().for_each(|(i, line)| {
        if i == 0 { return }

        line.iter().enumerate().for_each(|(j, c)| {
            if result[i - 1][j] == b'|' && *c == b'^' {
                split_count += 1;
            }
        });
    });

    println!("{:?}", split_count);

}

fn part_2() {

    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines: Vec<_> = contents.lines().collect(); // collect lines to avoid moving iterator
    let lines_bytes: Vec<Vec<u8>> = lines.iter().map(|x| x.as_bytes().to_vec()).collect();
    let mut memo_map: HashMap<(usize, usize), u64> = HashMap::new();

    let Some(start_col) = lines_bytes[0].iter().position(|x| *x == b'S') else { panic!("S Not found!!!")};
    println!("{:?}", check_split(&lines_bytes, 1, start_col, &mut memo_map));

}

fn check_split(matrix: &Vec<Vec<u8>>, i: usize, j: usize, memo_map: &mut HashMap<(usize, usize), u64>) -> u64 {
    let mut curr_row = i;

    while curr_row < matrix.len() {
        if matrix[curr_row][j] == b'^' {
            let memo = memo_map.get(&(curr_row, j));

            match memo {
                Some(val) => return *val,
                None => {
                    let result = check_split(&matrix, curr_row + 1, j - 1, memo_map) + check_split(&matrix, curr_row + 1, j + 1, memo_map);
                    memo_map.insert((curr_row, j), result);
                    return result;
                }
            }

        }

        curr_row += 1;
    }

    return 1;
}