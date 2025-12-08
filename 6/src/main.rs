use std::fs;

fn main() {
    part_1();
}

#[allow(dead_code)]
fn part_1() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();

    let split_lines = lines.map(|x| x.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();

    let num_cols = split_lines[0].len();
    let num_lines = split_lines.len();

    let mut result = 0;

    for col in 0..num_cols {
        let mut nums = Vec::new();
        let mut operator = "";

        for idx in 0..num_lines {
            match split_lines[idx][col] {
                "*" | "+" => operator = split_lines[idx][col],
                _ => nums.push(u64::from_str_radix(split_lines[idx][col], 10).unwrap()),
            }
        }
        result += match operator {
            "*" => nums.iter().fold(1, |acc, x| acc * x),
            "+" => nums.iter().fold(0, |acc, x| acc + x),
            _ => panic!("Unknown operator!")
        }

    }

    println!("{:?}", result);

}
