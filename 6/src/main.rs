use std::fs;
use std::ops::Range;

fn main() {
    part_2();
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

fn part_2() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines: Vec<_> = contents.lines().collect();

    let split_indices = find_indices_of_split(&lines);
    let ranges = convert_split_indices_to_range(split_indices, &lines);

    let result: Vec<_> = ranges.iter().map(|range| {
        let nums = get_nums_in_range(&lines, &range);
        let operator = get_operator_in_range(&lines, &range);
        return
            if operator == '+' { nums.iter().fold(0, |acc, x| acc + x) } 
            else { nums.iter().fold(1, |acc, x| acc * x) }
    }).collect();

    // println!("{:?}", result);
    println!("{:?}", result.iter().sum::<u64>());
}

fn find_indices_of_split(lines: &Vec<&str>) -> Vec<usize> {
    let mut result = Vec::new();

    let line_length = lines[0].len();

    for idx in 0..line_length {
        let is_separator = lines.iter().map(|line| line.as_bytes()[idx] as char).all(|f| f == ' ');
        if is_separator { result.push(idx); }
    }

    return  result;
}

fn convert_split_indices_to_range(split_indices: Vec<usize>, lines: &Vec<&str>) -> Vec<Range<usize>> {
    let mut result: Vec<_> = split_indices.iter().enumerate().map(|(idx, split)| {
        let start = if idx == 0 { 0 } else { split_indices[idx - 1] };
        return start..*split;
    }).collect();

    result.push(split_indices[split_indices.len() - 1]..lines[0].len());
    return result;
}

fn get_nums_in_range(lines: &Vec<&str>, range: &Range<usize>) -> Vec<u64> {
    let mut result = Vec::new();

    for j in range.start..range.end {
        let col_chars: String = lines.iter()
            .map(|line| line.as_bytes()[j] as char)
            .filter(|x| *x != ' ' && *x != '*' && *x != '+')
            .collect();

        if col_chars != "" {
            result.push(u64::from_str_radix(&col_chars, 10).unwrap());
        }
    }

    return result;
}

fn get_operator_in_range(lines: &Vec<&str>, range: &Range<usize>) -> char {
    if lines[lines.len() - 1][range.start..range.end].contains('*') {return '*'}
    return '+';
}