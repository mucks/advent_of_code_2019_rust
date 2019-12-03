use rayon::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug)]
struct Vector2 {
    pub x: i32,
    pub y: i32,
    pub steps: u32,
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let f = File::open("day_3/input").unwrap();
    let buf = BufReader::new(f);
    let wires: Vec<String> = buf.lines().filter_map(|l| l.ok()).collect();

    let a_paths: Vec<&str> = wires[0].split(",").collect();
    let b_paths: Vec<&str> = wires[1].split(",").collect();

    let a_coords = get_coords(a_paths);
    let b_coords = get_coords(b_paths);

    println!(
        "Day 3, part one: {}",
        part_one(a_coords.clone(), b_coords.clone())
    );
    println!("Day 3, part two: {}", part_two(a_coords, b_coords));
}

fn part_one(a_coords: Vec<Vector2>, b_coords: Vec<Vector2>) -> i32 {
    let mut new_a_coords: Vec<&Vector2> = a_coords
        .par_iter()
        .filter(|a_coord| b_coords.contains(a_coord))
        .collect();
    new_a_coords.sort_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())));
    new_a_coords[0].x + new_a_coords[0].y
}

fn part_two(a_coords: Vec<Vector2>, b_coords: Vec<Vector2>) -> u32 {
    let mut new_a_coords: Vec<&Vector2> = a_coords
        .par_iter()
        .filter(|a_coord| b_coords.contains(a_coord))
        .collect();
    let mut new_b_coords: Vec<&Vector2> = b_coords
        .par_iter()
        .filter(|b_coord| a_coords.contains(b_coord))
        .collect();
    new_a_coords.sort_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())));
    new_b_coords.sort_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())));

    let mut steps: Vec<u32> = Vec::new();

    for (a, b) in new_a_coords.iter().zip(new_b_coords) {
        steps.push(a.steps + b.steps);
    }
    steps.sort();
    steps[0]
}

fn get_coords(paths: Vec<&str>) -> Vec<Vector2> {
    let mut coords: Vec<Vector2> = Vec::new();
    let mut last = Vector2 {
        x: 0,
        y: 0,
        steps: 0,
    };
    for path in paths {
        let mut current = last.clone();

        let chars: Vec<char> = path.chars().collect();
        let (direction, step_str) = chars.split_at(1);
        let step: i32 = step_str.iter().collect::<String>().parse().unwrap();
        current.steps += step as u32;
        match direction[0] {
            'U' => current.y -= step,
            'D' => current.y += step,
            'R' => current.x += step,
            'L' => current.x -= step,
            _ => {}
        }

        let points: Vec<Vector2> = if last.x <= current.x {
            (last.x..current.x)
        } else {
            (current.x..last.x)
        }
        .enumerate()
        .map(|(i, x)| Vector2 {
            x: x,
            y: current.y,
            steps: last.steps + i as u32,
        })
        .collect();
        coords.extend(points);

        let points: Vec<Vector2> = if last.y <= current.y {
            (last.y..current.y)
        } else {
            (current.y..last.y)
        }
        .enumerate()
        .map(|(i, y)| Vector2 {
            x: current.x,
            y: y,
            steps: last.steps + i as u32,
        })
        .collect();
        coords.extend(points);

        last = current;
    }
    coords
}
