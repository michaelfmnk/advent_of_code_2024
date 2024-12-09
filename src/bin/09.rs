use std::ops::Div;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u128> {
    let expanded = expand(input).ok()?;
    println!("{}", expanded);

    let compacted = compact(&expanded);
    println!("{}", compacted);

    let result = checksum(&compacted);
    println!("{}", result);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn expand(input: &str) -> Result<String, String> {
    let mut result = String::new();
    for (i, ch) in input.chars().enumerate() {
        if let Some(count) = ch.to_digit(10) {
            if i % 2 == 0 {
                result.push_str(&i.div(2).to_string().repeat(count as usize));
            } else {
                result.push_str(&".".repeat(count as usize));
            }
        } else {
            return Err(format!("Invalid character in input: {}", ch));
        }
    }
    Ok(result)
}

fn compact(input: &str) -> String {
    let mut result = String::from(input);
    let dots_count = result.matches('.').count();

    for ch in input.chars().rev() {
        if ch.is_digit(10) {
            result = result.replacen('.', &ch.to_string(), 1);
        }
    }
    result.replace_range(result.len() - dots_count.., ".".repeat(dots_count).as_str());
    result
}

fn checksum(input: &str) -> u128 {
    let mut result: u128 = 0;
    for (i, ch) in input.chars().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            if let Some(product) = (digit as u128).checked_mul(i as u128) {
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
}
