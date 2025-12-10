use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // part_1();
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
                .map(|pos| usize::from_str_radix(pos, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let sorted_distances = get_sorted_distances(&lines);

    let ((box1, box2), dist) = sorted_distances[0];
    println!("Box 1: {:?}, Box 2: {:?}, Dist: {:?}", box1, box2, dist);
}

fn part_2() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let vertices: Vec<_> = contents
        .lines()
        .map(|line| {
            line.split(",")
                .map(|pos| usize::from_str_radix(pos, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let (x_map, y_map) = get_space_map(&vertices);
    let (x_map_reverse, y_map_reverse) = get_reverse_space_map(&x_map, &y_map);

    let new_vertices = vertices
        .iter()
        .map(|line| (x_map[&line[0]], y_map[&line[1]]))
        .collect::<Vec<_>>();

    let mut grid: Vec<Vec<u8>> = vec![vec![b' '; x_map.len()]; y_map.len()];

    fill_edges(&mut grid, &new_vertices);
    fill_inside(&mut grid);

    let sorted_distances =
        get_sorted_distances_with_space_map(&new_vertices, &x_map_reverse, &y_map_reverse);

    let max_dist = sorted_distances
        .iter()
        .find(|((box1, box2), _)| {
            let (x1, y1) = new_vertices[*box1];
            let (x2, y2) = new_vertices[*box2];
            check_contiguous(&grid, x1, y1, x2, y2)
        })
        .unwrap();

    let ((box1, box2), dist) = max_dist;
    println!(
        "Box 1: {:?}, Box 2: {:?}, Dist: {:?}",
        vertices[*box1], vertices[*box2], dist
    );
}

/**
 * Maps the x and y -> indices.
 * Reduces the space of the problem by mapping the x and y values to their indices.
 */
fn get_space_map(vertices: &Vec<Vec<usize>>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let x_set = vertices.iter().map(|line| line[0]).collect::<HashSet<_>>();
    let mut x_values = x_set.iter().collect::<Vec<_>>();
    x_values.sort_by_key(|x| *x);
    let x_map: HashMap<usize, usize> =
        HashMap::from_iter(x_values.iter().enumerate().map(|(idx, x)| (**x, idx)));

    let y_set = vertices.iter().map(|line| line[1]).collect::<HashSet<_>>();
    let mut y_values = y_set.iter().collect::<Vec<_>>();
    y_values.sort_by_key(|y| *y);
    let y_map: HashMap<usize, usize> =
        HashMap::from_iter(y_values.iter().enumerate().map(|(idx, y)| (**y, idx)));

    return (x_map, y_map);
}

fn get_reverse_space_map(
    x_map: &HashMap<usize, usize>,
    y_map: &HashMap<usize, usize>,
) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let x_map_reverse: HashMap<usize, usize> = x_map.iter().map(|(k, v)| (*v, *k)).collect();
    let y_map_reverse: HashMap<usize, usize> = y_map.iter().map(|(k, v)| (*v, *k)).collect();
    return (x_map_reverse, y_map_reverse);
}

fn check_contiguous(grid: &Vec<Vec<u8>>, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let x_range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
    let y_range = if y1 < y2 { y1..=y2 } else { y2..=y1 };

    for x in x_range {
        for y in y_range.clone() {
            if grid[y][x] == b' ' {
                return false;
            }
        }
    }
    true
}

fn get_sorted_distances(lines: &Vec<Vec<usize>>) -> Vec<((usize, usize), u64)> {
    let mut distances: HashMap<(usize, usize), u64> = HashMap::new();
    for (i, pos) in lines.iter().enumerate() {
        let mut j = 0;
        while j < i {
            let dist = calculate_distance(pos, &lines[j]);
            distances.insert((i, j), dist);
            j += 1;
        }
    }

    let mut sorted_distances = distances.into_iter().collect::<Vec<_>>();
    sorted_distances.sort_by_key(|(_, v)| -(*v as i64));
    return sorted_distances;
}

fn get_sorted_distances_with_space_map(
    vertices: &Vec<(usize, usize)>,
    x_map_reverse: &HashMap<usize, usize>,
    y_map_reverse: &HashMap<usize, usize>,
) -> Vec<((usize, usize), u64)> {
    let mut distances: HashMap<(usize, usize), u64> = HashMap::new();
    for (i, pos) in vertices.iter().enumerate() {
        let mut j = 0;
        while j < i {
            let p1 = vec![x_map_reverse[&pos.0], y_map_reverse[&pos.1]];
            let p2 = vec![x_map_reverse[&vertices[j].0], y_map_reverse[&vertices[j].1]];
            let dist = calculate_distance(&p1, &p2);
            distances.insert((i, j), dist);
            j += 1;
        }
    }

    let mut sorted_distances = distances.into_iter().collect::<Vec<_>>();
    sorted_distances.sort_by_key(|(_, v)| -(*v as i64));
    return sorted_distances;
}

fn calculate_distance(p1: &Vec<usize>, p2: &Vec<usize>) -> u64 {
    return (((p1[0] as i64 - p2[0] as i64).abs() + 1) * ((p1[1] as i64 - p2[1] as i64).abs() + 1))
        as u64;
}

fn fill_edges(grid: &mut Vec<Vec<u8>>, lines: &Vec<(usize, usize)>) {
    let last_pos = &lines[lines.len() - 1];
    let mut last_pos_x = last_pos.0;
    let mut last_pos_y = last_pos.1;

    lines.iter().for_each(|(x, y)| {
        let last_x = last_pos_x;
        let last_y = last_pos_y;

        let x_range = if last_x < *x {
            last_x..=*x
        } else {
            *x..=last_x
        };
        let y_range = if last_y < *y {
            last_y..=*y
        } else {
            *y..=last_y
        };

        for j in y_range {
            for i in x_range.clone() {
                if (i == *x && j == *y) || (i == last_x && j == last_y) {
                    grid[j][i] = b'#';
                } else {
                    grid[j][i] = b'.';
                }
            }
        }

        last_pos_x = *x;
        last_pos_y = *y;
    });
}

fn fill_inside(grid: &mut Vec<Vec<u8>>) {
    // we can assume that the point right below the edge in the first row is within the shape.
    // therefore, we only need to flood-fill from that point, filling all spaces with dots.
    let x = grid[0].iter().position(|c| *c == b'#').unwrap();
    fill_from_point(grid, 1, x + 1);
}

fn fill_from_point(grid: &mut Vec<Vec<u8>>, y: usize, x: usize) {
    let mut stack: Vec<(usize, usize)> = vec![(y, x)];

    while !stack.is_empty() {
        let (y, x) = stack.pop().unwrap();

        if x >= grid[0].len() || y >= grid.len() || grid[y][x] != b' ' {
            continue;
        }

        grid[y][x] = b'.';

        if x > 0 {
            stack.push((y, x - 1));
        }
        if y > 0 {
            stack.push((y - 1, x));
        }
        stack.push((y, x + 1));
        stack.push((y + 1, x));
    }
}
