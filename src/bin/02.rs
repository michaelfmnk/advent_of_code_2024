advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let result = read_input(input)
        .into_iter()
        .filter(|report| is_safe(report))
        .count();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let result = read_input(input)
        .into_iter()
        .filter(|report| is_safe_skipping_one(report))
        .count();
    Some(result)
}

fn is_safe(report: &[i32]) -> bool {
    let check_asc = || report.is_sorted_by(|a, b| a < b && a.abs_diff(*b) <= 3);
    let check_desc = || report.is_sorted_by(|a, b| a > b && a.abs_diff(*b) <= 3);
    check_asc() || check_desc()
}

fn is_safe_skipping_one(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for skip in 0..report.len() {
        let slice: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip)
            .map(|(_, &num)| num)
            .collect();

        if is_safe(&slice) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
