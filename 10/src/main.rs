use regex::Regex;
use std::fs;

fn main() {
    part_1();
    // part_2();
}

#[derive(Debug)]
struct Machine {
    desired_lights: u16,
    buttons: Vec<u16>,
}

#[allow(dead_code)]
fn part_1() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");
    let machines = process_input(&contents);

    let results = machines
        .iter()
        .map(|machine| find_button_presses(machine).unwrap())
        .sum::<usize>();
    println!("{:?}", results);
}

fn find_button_presses(machine: &Machine) -> Option<usize> {
    let combinations = find_combinations(&machine.buttons);

    for combination in combinations {
        let result = combination.iter().fold(0, |acc, x| acc ^ x);
        if result == machine.desired_lights {
            return Some(combination.len());
        }
    }
    None
}

fn find_combinations(arr: &Vec<u16>) -> Vec<Vec<u16>> {
    let mut result = Vec::new();
    for i in 1..=arr.len() {
        result.extend(find_combinations_of_n_elements(&arr, i));
    }
    result
}

fn find_combinations_of_n_elements(arr: &Vec<u16>, num_elements: usize) -> Vec<Vec<u16>> {
    let mut arr = arr.clone();
    if arr.len() == num_elements {
        return vec![arr];
    }

    if num_elements == 1 {
        return arr.iter().map(|x| vec![*x]).collect();
    }

    let first_element: Vec<_> = arr.splice(0..1, []).collect();

    let mut comb = find_combinations_of_n_elements(&arr, num_elements - 1)
        .iter()
        .map(|x| {
            let mut result = first_element.clone();
            result.extend(x);
            result
        })
        .collect::<Vec<_>>();

    comb.extend(find_combinations_of_n_elements(&arr, num_elements));
    comb
}

fn process_input(contents: &str) -> Vec<Machine> {
    let lines = contents.lines();
    lines
        .map(|line| {
            let re = Regex::new(r"\[(.*)\] (\(.*\))+ \{(.*)\}").unwrap();
            let captures = re.captures(line).unwrap();
            let desired_lights = &captures[1]
                .as_bytes()
                .iter()
                .map(|x| if *x == b'#' { true } else { false })
                .collect::<Vec<bool>>();

            let buttons = captures[2]
                .split(" ")
                .map(|s| s.trim_matches(|c| c == '(' || c == ')'))
                .map(|s| {
                    s.split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>();

            Machine {
                buttons: buttons.iter().map(|x| find_u16_from_usize_vec(x)).collect(),
                desired_lights: find_u16_from_bool_vec(desired_lights),
            }
        })
        .collect()
}

/**
 * first item in vector is the least significant bit
 */
fn find_u16_from_bool_vec(bool_vec: &Vec<bool>) -> u16 {
    bool_vec
        .iter()
        .rev()
        .fold(0, |acc, x| acc * 2 + if *x { 1 } else { 0 })
}

fn find_u16_from_usize_vec(vec: &Vec<usize>) -> u16 {
    vec.iter().fold(0, |acc, x| acc + 2u16.pow(*x as u32))
}
