use std::{collections::HashMap, fs};

fn main() {
    part_1();
    part_2();
}

#[derive(Debug, Clone)]
struct Device {
    label: String,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct RelevantVisits {
    dac: bool,
    fft: bool,
}

#[allow(dead_code)]
fn part_1() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let devices = process_input(&contents);
    let start = &devices["you"];

    println!("Count: {:?}", dfs_count_start(start, &devices));
}

fn part_2() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let devices = process_input(&contents);
    let start = &devices["svr"];
    let mut memo_map: HashMap<String, u64> = HashMap::new();

    println!(
        "Count: {:?}",
        dfs_count_start_pt2(
            start,
            &devices,
            &mut RelevantVisits {
                dac: false,
                fft: false
            },
            &mut memo_map
        )
    );
}

fn dfs_count_start(device: &Device, devices: &HashMap<String, Device>) -> u64 {
    if device.label == "out" {
        return 1;
    }

    device.outputs.iter().fold(0, |acc, x| {
        let next = &devices[x];
        acc + dfs_count_start(next, devices)
    })
}

fn dfs_count_start_pt2(
    device: &Device,
    devices: &HashMap<String, Device>,
    relevant_visits: &mut RelevantVisits,
    memo_map: &mut HashMap<String, u64>,
) -> u64 {
    let mut key = device.label.clone();
    key.push_str(&relevant_visits.dac.to_string());
    key.push_str(&relevant_visits.fft.to_string());

    if memo_map.contains_key(&key) {
        return memo_map[&key];
    }

    if device.label == "out" {
        if relevant_visits.dac && relevant_visits.fft {
            memo_map.insert(key.clone(), 1);
            return 1;
        }

        memo_map.insert(key.clone(), 0);
        return 0;
    }

    if device.label == "dac" {
        relevant_visits.dac = true;
    }

    if device.label == "fft" {
        relevant_visits.fft = true;
    }

    let result = device.outputs.iter().fold(0, |acc, x| {
        let next = &devices[x];
        acc + dfs_count_start_pt2(next, devices, &mut relevant_visits.clone(), memo_map)
    });

    memo_map.insert(key.clone(), result);
    return result;
}

fn process_input(contents: &str) -> HashMap<String, Device> {
    let lines = contents.lines();
    let mut map = lines
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let device = Device {
                label: parts[0].to_string(),
                outputs: parts[1].split(" ").map(|x| x.to_string()).collect(),
            };
            (device.label.clone(), device)
        })
        .collect::<HashMap<String, Device>>();

    map.insert(
        "out".to_string(),
        Device {
            label: "out".to_string(),
            outputs: vec![],
        },
    );

    map
}
