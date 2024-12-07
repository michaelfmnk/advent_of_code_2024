advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let allowed_operations = vec![|a, b| a + b, |a, b| a * b];
    calculate_result(input, &allowed_operations)
}

pub fn part_two(input: &str) -> Option<u64> {
    let allowed_operations = vec![|a, b| a + b, |a, b| a * b, concat_integers];
    calculate_result(input, &allowed_operations)
}

fn calculate_result(input: &str, allowed_operations: &[fn(u64, u64) -> u64]) -> Option<u64> {
    let input = read_input(input);

    let result = input
        .iter()
        .filter(|(target, values)| {
            let first = *values.first().unwrap();
            can_result_in(*target, first, &values[1..], allowed_operations)
        })
        .map(|(target, _)| *target)
        .sum();
    Some(result)
}

fn can_result_in(
    target: u64,
    acc: u64,
    values: &[u64],
    operations: &[fn(u64, u64) -> u64],
) -> bool {
    let mut stack = vec![(acc, values)];

    while let Some((acc, values)) = stack.pop() {
        if values.is_empty() {
            if acc == target {
                return true;
            }
            continue;
        }

        for op in operations {
            let new_val = op(acc, values[0]);
            stack.push((new_val, &values[1..]));
        }
    }

    false
}

fn concat_integers(a: u64, b: u64) -> u64 {
    let mut result = a * 10u64.pow(b.to_string().len() as u32);
    result += b;
    result
}

fn read_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter(|line| line.contains(": "))
        .map(|line| {
            let mut parts = line.split(": ");
            let depth = parts.next().unwrap().parse().unwrap();
            let range = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();
            (depth, range)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 11387);
    }
}
