use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    part_2();
}

#[allow(dead_code)]
fn part_1() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents
        .lines()
        .map(|line| {
            line.split(",")
                .map(|pos| i64::from_str_radix(pos, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut distances: HashMap<(usize, usize), i64> = HashMap::new();
    lines.iter().enumerate().for_each(|(idx, pos)| {
        let mut cmp_idx = idx + 1;
        while cmp_idx < lines.len() {
            let dist = calculate_distance(pos, &lines[cmp_idx]);
            distances.insert((idx, cmp_idx), dist);
            cmp_idx += 1;
        }
    });

    let mut sorted_dists = distances.iter().collect::<Vec<_>>();
    sorted_dists.sort_by_key(|(_, v)| *v);

    let mut circuits: Vec<HashSet<usize>> =
        (0..lines.len()).map(|idx| HashSet::from([idx])).collect();

    for i in 0..1_000 {
        let ((box1, box2), _) = sorted_dists[i];
        let circuit1 = circuits
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(box1))
            .unwrap();
        let circuit2 = circuits
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(box2))
            .unwrap();

        let union = circuit1
            .1
            .union(circuit2.1)
            .copied()
            .collect::<HashSet<_>>();

        // To avoid double-borrow, update after both lookups.
        let c1_idx = circuit1.0;
        let c2_idx = circuit2.0;
        circuits[c1_idx] = union;
        if c1_idx != c2_idx {
            circuits[c2_idx].clear();
        }
    }

    circuits.sort_by_key(|x| x.len());
    circuits.reverse();
    let mult = circuits[0..3]
        .iter()
        .map(|x| x.len())
        .fold(1u64, |acc, x| acc * (x as u64));

    println!("{:?}", mult);
}

fn part_2() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents
        .lines()
        .map(|line| {
            line.split(",")
                .map(|pos| i64::from_str_radix(pos, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut distances: HashMap<(usize, usize), i64> = HashMap::new();
    lines.iter().enumerate().for_each(|(idx, pos)| {
        let mut cmp_idx = idx + 1;
        while cmp_idx < lines.len() {
            let dist = calculate_distance(pos, &lines[cmp_idx]);
            distances.insert((idx, cmp_idx), dist);
            cmp_idx += 1;
        }
    });

    let mut sorted_dists = distances.iter().collect::<Vec<_>>();
    sorted_dists.sort_by_key(|(_, v)| *v);

    let mut circuits: Vec<HashSet<usize>> =
        (0..lines.len()).map(|idx| HashSet::from([idx])).collect();

    for i in 0..sorted_dists.len() {
        let ((box1, box2), _) = sorted_dists[i];
        let circuit1 = circuits
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(box1))
            .unwrap();
        let circuit2 = circuits
            .iter()
            .enumerate()
            .find(|(_, set)| set.contains(box2))
            .unwrap();

        let union = circuit1
            .1
            .union(circuit2.1)
            .copied()
            .collect::<HashSet<_>>();

        // To avoid double-borrow, update after both lookups.
        let c1_idx = circuit1.0;
        let c2_idx = circuit2.0;
        circuits[c1_idx] = union;
        if c1_idx != c2_idx {
            circuits[c2_idx].clear();
        }

        let num_of_circuits = circuits.iter().filter(|x| x.len() > 0).count();
        if num_of_circuits == 1 {
            let x_mult = lines[*box1][0] * lines[*box2][0];
            println!("Multiplication of x: {:?}", x_mult);
            return;
        }
    }

    circuits.sort_by_key(|x| x.len());
    circuits.reverse();
    let mult = circuits[0..3]
        .iter()
        .map(|x| x.len())
        .fold(1u64, |acc, x| acc * (x as u64));

    println!("{:?}", mult);
}

fn calculate_distance(p1: &Vec<i64>, p2: &Vec<i64>) -> i64 {
    return ((p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) + (p1[2] - p2[2]).pow(2)).isqrt();
}
