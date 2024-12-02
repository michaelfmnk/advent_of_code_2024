use std::fmt::{Display, Formatter};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = read_input(input)
        .into_iter()
        .filter(|report| is_safe(report))
        .count();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = read_input(input)
        .into_iter()
        .filter(|report| is_safe_skipping_one(&report))
        .count();
    Some(result as u32)
}

fn is_safe<'i, T: IntoIterator<Item = &'i i32>>(levels: T) -> bool {
    use itertools::Itertools;
    let mut levels_iter = levels.into_iter().tuple_windows().peekable();
    let Some((first, second)) = levels_iter.peek() else {
        return true;
    };
    let is_increasing = second > first;
    levels_iter.all(|(first, second)| {
        let diff = second.abs_diff(*first);
        diff >= 1 && diff <= 3 && (second > first) == is_increasing
    })
}


fn is_safe_skipping_one(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for skip in 0..report.len() {
        let levels_iter = report
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != skip)
            .map(|(_, level)| level);

        if is_safe(levels_iter) {
            return true;
        }
    }
    false
}

fn read_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

struct SafetyReport {
    result: Vec<bool>,
}

impl Display for SafetyReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY)).unwrap();
        println!("{}", result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        println!("{}", result.unwrap());
    }
}
