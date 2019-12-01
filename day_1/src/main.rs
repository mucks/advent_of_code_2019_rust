use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_masses() -> Vec<usize> {
    let f = File::open("day_1/input").unwrap();
    let f = BufReader::new(f);

    f.lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<usize>().ok())
        .collect()
}

fn part_one() -> usize {
    get_masses()
        .into_iter()
        .map(|m| (f32::floor(m as f32 / 3.0) - 2.0) as usize)
        .sum()
}

fn part_two() -> usize {
    get_masses().into_iter().map(|m| get_fuel(m)).sum()
}

fn get_fuel(mass: usize) -> usize {
    let mut fuel = mass as f32;
    let mut total_fuel = 0.0;
    while fuel > 0.0 {
        fuel = f32::floor(fuel as f32 / 3.0) - 2.0;
        if fuel > 0.0 {
            total_fuel += fuel;
        }
    }
    total_fuel as usize
}

fn main() {
    println!("Day 1, part one: {}", part_one());
    println!("Day 1, part two: {}", part_two());
}
