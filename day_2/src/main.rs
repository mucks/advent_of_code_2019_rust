use std::fs::File;
use std::io::Read;

fn get_values() -> Vec<usize> {
    let mut f = File::open("day_2/input").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s.split(",").filter_map(|s| s.parse().ok()).collect()
}

fn part_one(values: Vec<usize>) -> usize {
    calculate(values, 12, 2)
}

fn calculate(mut values: Vec<usize>, noun: usize, verb: usize) -> usize {
    values[1] = noun;
    values[2] = verb;

    for i in (0..values.len()).step_by(4) {
        let op = values[i];
        let a_pos = values[i + 1];
        let b_pos = values[i + 2];
        let result_pos = values[i + 3];

        values[result_pos] = match op {
            1 => values[a_pos] + values[b_pos],
            2 => values[a_pos] * values[b_pos],
            99 => break,
            _ => panic!("something went wrong"),
        };
    }
    values[0]
}

fn part_two(values: Vec<usize>) -> usize {
    let result = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            if result == calculate(values.clone(), noun, verb) {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn main() {
    let values = get_values();
    println!("Day 2, part one: {}", part_one(values.clone()));
    println!("Day 2, part two: {}", part_two(values));
}
