use std::ops::Div;
use std::string::String;
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u128> {
    let start = std::time::Instant::now();
    let expanded = expand(input).ok()?;
    println!("expand took {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let compacted = compact(&expanded);
    println!("compact took {:?}", start.elapsed());

    let start = std::time::Instant::now();
    let result = checksum(&compacted);
    println!("checksum took {:?}", start.elapsed());

    println!("{}", result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn expand(input: &str) -> Result<Vec<Option<usize>>, String> {
    let mut result = Vec::new();
    for (i, ch) in input.chars().enumerate() {
        if let Some(count) = ch.to_digit(10) {
            if i % 2 == 0 {
                for _ in 0..count {
                    result.push(Some(i.div(2)));
                }
            } else {
                for _ in 0..count {
                    result.push(None);
                }
            }
        } else {
            return Err(format!("Invalid character in input: {}", ch));
        }
    }
    Ok(result)
}

fn compact(input: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut result = input.clone();
    let mut empty_space_positions = result.iter().positions(|&ch| ch.is_none()).rev().collect_vec();
    let empty_space = empty_space_positions.len();

    for ch in input.iter().rev() {
        if let Some(ch) = ch {
            if let Some(position) = empty_space_positions.pop() {
                result[position] = Some(*ch);
            } else {
                break;
            }
        }
    }

    result.truncate(input.len() - empty_space);
    result
}

fn checksum(input: &[Option<usize>]) -> u128 {
    let mut result: u128 = 0;

    for (i, ch) in input.iter().enumerate() {
        if let Some(ch) = ch {
            if let Some(product) = (*ch as u128).checked_mul(i as u128) {
                if let Some(sum) = result.checked_add(product) {
                    result = sum;
                } else {
                    panic!("Overflow occurred during addition");
                }
            } else {
                panic!("Overflow occurred during multiplication");
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
