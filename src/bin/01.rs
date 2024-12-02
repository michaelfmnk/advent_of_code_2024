use advent_of_code::solution;
use std::collections::HashMap;

solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (a, b) = parse_input(input);
    let result = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (list1, list2) = parse_input(input);
    let list2_freq = counts(list2);

    let result = list1
        .iter()
        .map(|x| *x * list2_freq.get(&x).unwrap_or(&0))
        .sum();

    Some(result)
}

fn counts(vec: Vec<i32>) -> HashMap<i32, i32> {
    let mut freq = HashMap::new();
    for num in vec.iter() {
        *freq.entry(*num).or_insert(0) += 1;
    }
    freq
}

fn parse_input(text: &str) -> (Vec<i32>, Vec<i32>) {
    text.lines()
        .map(|line| {
            line.split("   ")
                .map(|num| num.parse().unwrap_or(0))
                .collect::<Vec<i32>>()
        })
        .map(|v| (v[0], v[1]))
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(309560));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        println!("{}", result.unwrap());
        assert_eq!(result, Some(76056));
    }
    
}
