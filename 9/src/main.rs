use std::{collections::HashMap, fs};

fn main() {
    part_1();
}

#[allow(dead_code)]
fn part_1() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents
        .lines()
        .map(|line| {
            line.split(",")
                .map(|pos| u64::from_str_radix(pos, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut distances: HashMap<(usize, usize), u64> = HashMap::new();

    for (i, pos) in lines.iter().enumerate() {
        let mut j = 0;
        while j < i {
            let dist = calculate_distance(pos, &lines[j]);
            distances.insert((i, j), dist);
            j += 1;
        }
    }

    let mut sorted_distances = distances.iter().collect::<Vec<_>>();
    sorted_distances.sort_by_key(|(_, v)| *v);

    let ((box1, box2), dist) = sorted_distances[sorted_distances.len() - 1];
    println!("Box 1: {:?}, Box 2: {:?}, Dist: {:?}", box1, box2, dist);
}

fn calculate_distance(p1: &Vec<u64>, p2: &Vec<u64>) -> u64 {
    return (((p1[0] as i64 - p2[0] as i64).abs() + 1) * ((p1[1] as i64 - p2[1] as i64).abs() + 1))
        as u64;
}
