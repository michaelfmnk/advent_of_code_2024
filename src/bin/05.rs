use itertools::Itertools;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet, VecDeque};
use std::num::ParseIntError;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules = parse_rules(input).ok()?;
    // sort rules by the first number
    let rules = rules.into_iter().sorted_by_key(|(x, _)| *x).collect_vec();

    let instructions_sets = parse_instructions(input).ok()?;

    let result = instructions_sets
        .iter()
        .filter(|instructions| is_valid_instruction_set(&rules, instructions))
        .map(|it| it[it.len() / 2])
        .sum();

    Some(result)
}

fn parse_rules(input: &str) -> Result<Vec<(u32, u32)>, ParseIntError> {
    input
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let mut parts = line.split("|");
            let x = parts.next().unwrap().parse()?;
            let y = parts.next().unwrap().parse()?;
            Ok((x, y))
        })
        .collect()
}

fn parse_instructions(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| line.split(",").map(|x| x.parse()).collect())
        .collect()
}

fn is_valid_instruction_set(rules: &[(u32, u32)], instructions: &[u32]) -> bool {
    let positions = get_positions(instructions);
    rules.iter().all(|&(x, y)| {
        if let (Some(x_pos), Some(y_pos)) = (positions.get(&x), positions.get(&y)) {
            x_pos < y_pos
        } else {
            true
        }
    })
}

fn get_positions(vec: &[u32]) -> HashMap<u32, usize> {
    vec.iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, &x)| {
            acc.insert(x, i);
            acc
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = parse_rules(input).ok()?;
    let instructions_sets = parse_instructions(input).ok()?;

    let result = instructions_sets
        .iter()
        .filter(|instructions| !is_valid_instruction_set(&rules, instructions))
        .map(|it| fix_instruction_set(&rules, it))
        .map(|it| it[it.len() / 2])
        .sum();
    Some(result)
}

fn fix_instruction_set(rules: &[(u32, u32)], instructions: &[u32]) -> Vec<u32> {
    let comparator = |a: &u32, b: &u32| {
        rules
            .iter()
            .find(|rule| has_value(rule, *a) && has_value(rule, *b))
            .map_or(Equal, |r| if *a == r.0 { Less } else { Greater })
    };

    let mut sorted_instructions = instructions.to_vec();
    sorted_instructions.sort_by(comparator);
    sorted_instructions
}
fn has_value(rule: &(u32, u32), value: u32) -> bool {
    rule.0 == value || rule.1 == value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 123);
    }
}
