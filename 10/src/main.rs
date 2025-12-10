use regex::Regex;
use std::fs;
use z3::ast::Int;
use z3::{Optimize, SatResult};

fn main() {
    // part_1();
    part_2();
}

#[derive(Debug)]
struct Machine {
    desired_lights: u16,
    buttons: Vec<u16>,
    buttons_indices: Vec<Vec<usize>>,
    joltages: Vec<u16>,
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

fn part_2() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");
    let machines = process_input(&contents);

    let results = machines
        .iter()
        .map(|machine| find_button_presses_for_joltage(machine))
        .sum::<u64>();
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

fn find_button_presses_for_joltage(machine: &Machine) -> u64 {
    let optimize = Optimize::new();

    let button_pressed_n = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(idx, _)| Int::new_const(format!("button_{idx}")))
        .collect::<Vec<_>>();

    for (idx, joltage) in machine.joltages.iter().enumerate() {
        let relevant_button_vars = machine
            .buttons_indices
            .iter()
            .enumerate()
            .filter(|(_, vals)| vals.contains(&idx))
            .map(|(btn_idx, _)| &button_pressed_n[btn_idx])
            .collect::<Vec<_>>();

        let eq_result = Int::from_u64(*joltage as u64);
        let sum = relevant_button_vars
            .iter()
            .fold(Int::from_u64(0), |acc, x| acc + *x);
        optimize.assert(&sum.eq(&eq_result));
    }

    button_pressed_n
        .iter()
        .for_each(|x| optimize.assert(&x.ge(&Int::from_u64(0))));

    // Minimize the sum of all button_pressed_n values
    let total_sum = button_pressed_n
        .iter()
        .fold(Int::from_u64(0), |acc, x| acc + x);
    optimize.minimize(&total_sum);

    let result: u64 = match optimize.check(&[]) {
        SatResult::Sat => {
            let model = optimize.get_model().unwrap();
            button_pressed_n
                .iter()
                .map(|x| model.eval(x, true).unwrap().as_u64().unwrap())
                .sum::<u64>()
        }
        SatResult::Unsat => {
            panic!("No solution found");
        }
        SatResult::Unknown => {
            panic!("Solver returned unknown");
        }
    };
    println!("Solution: {:?}", result);
    result
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

            let joltages: Vec<_> = captures[3]
                .split(",")
                .map(|x| x.parse::<u16>().unwrap())
                .collect();

            Machine {
                buttons: buttons.iter().map(|x| find_u16_from_usize_vec(x)).collect(),
                buttons_indices: buttons,
                desired_lights: find_u16_from_bool_vec(desired_lights),
                joltages,
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
