use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u128> {
    solve(parse_input(input))
}

pub fn part_two(input: &str) -> Option<u128> {
    let input: Vec<_> = parse_input(input)
        .iter()
        .map(|(a, b, prize)| {
            (
                (a.0, a.1),
                (b.0, b.1),
                (prize.0 + 10000000000000, prize.1 + 10000000000000),
            )
        })
        .collect();

    solve(input)
}

fn is_plausible((x, y): &(f64, f64)) -> bool {
    // is both close to an integer and positive
    let nearest_x = x.round();
    let nearest_y = y.round();

    (nearest_x - x).abs() < 0.01
        && (nearest_y - y).abs() < 0.01
        && nearest_x > 0.0
        && nearest_y > 0.0
}

fn solve(input: Vec<((u128, u128), (u128, u128), (u128, u128))>) -> Option<u128> {
    let mut result = 0;
    for (a, b, prize) in input {
        let res = solve_machine(a, b, prize);
        if let Some((x, y)) = res {
            result += 3 * x + y;
        }
    }

    Some(result)
}

fn solve_machine(a: (u128, u128), b: (u128, u128), prize: (u128, u128)) -> Option<(u128, u128)> {
    let a_x = a.0 as i128;
    let a_y = a.1 as i128;
    let b_x = b.0 as i128;
    let b_y = b.1 as i128;
    let p_x = prize.0 as i128;
    let p_y = prize.1 as i128;

    let a = (-b_x * p_y + b_y * p_x) as f64 / (a_x * b_y - a_y * b_x) as f64;
    let b = (a_x * p_y - a_y * p_x) as f64 / (a_x * b_y - a_y * b_x) as f64;

    if is_plausible(&(a, b)) {
        Some((a as u128, b as u128))
    } else {
        None
    }
}

fn parse_input(input: &str) -> Vec<((u128, u128), (u128, u128), (u128, u128))> {
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut prize = Vec::new();

    for line in input.lines() {
        if line.starts_with("Button A: ") {
            a.push(
                line.split(", ")
                    .map(|part| part.split("+").last().unwrap().parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        } else if line.starts_with("Button B: ") {
            b.push(
                line.split(", ")
                    .map(|part| part.split("+").last().unwrap().parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        } else if line.starts_with("Prize: ") {
            prize.push(
                line.split(", ")
                    .map(|part| part.split("=").last().unwrap().parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        }
    }

    a.into_iter()
        .zip(b)
        .zip(prize)
        .map(|((a, b), prize)| (a, b, prize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 875318608908);
    }
}
