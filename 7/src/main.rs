use std::fs;

fn main() {
    part_1();
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