use std::collections::HashMap;
use std::ops::Div;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let stones = read_input(input);
    Some(compute(stones, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones = read_input(input);
    Some(compute(stones, 75))
}

fn read_input(input: &str) -> Vec<u128> {
    input
        .lines()
        .take(1)
        .flat_map(|line| line.split_whitespace())
        .map(|num| num.parse().unwrap())
        .collect()
}

fn compute(stones: Vec<u128>, blinks: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|num| dfs(*num, 0, blinks, &mut cache))
        .sum()
}

fn dfs(num: u128, depth: usize, limit: usize, cache: &mut HashMap<(u128, usize), usize>) -> usize {
    if depth == limit {
        return 1;
    }

    if let Some(&result) = cache.get(&(num, depth)) {
        return result;
    }

    let result = if num == 0 {
        dfs(1, depth + 1, limit, cache)
    } else if has_even_number_of_digits(num) {
        let (r, l) = split_number(num);
        dfs(r, depth + 1, limit, cache) + dfs(l, depth + 1, limit, cache)
    } else {
        dfs(num * 2024, depth + 1, limit, cache)
    };

    cache.insert((num, depth), result);
    result
}

fn split_number(num: u128) -> (u128, u128) {
    // count the number of digits in the number
    let mut digit_count = 0;
    let mut temp = num;
    while temp > 0 {
        temp /= 10;
        digit_count += 1;
    }

    // find the divisor for splitting the number
    let half_digits = digit_count / 2;
    let divisor = 10_u128.pow(half_digits as u32);

    // split the number
    let second_half = num % divisor; // extract the second half
    let first_half = num / divisor; // extract the first half

    (first_half, second_half)
}

fn has_even_number_of_digits(num: u128) -> bool {
    let mut num = num;
    let mut count = 0;
    while num > 0 {
        num = num.div(10);
        count += 1;
    }
    count % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 189541);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 226596360258785);
    }
}
