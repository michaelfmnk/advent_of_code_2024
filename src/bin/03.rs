advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let result = extract_tuples(input).iter()
        .map(|(x, y)| x * y)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = fancy_regex::Regex::new(r"don't\(\).*?(?=do\(\))").unwrap();
    let clean_input = input.replace('\n', "");
    let clean_input = re.replace_all(&clean_input, "").to_string();
    part_one(&clean_input)
}

fn extract_tuples(input: &str) -> Vec<(u32, u32)> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            let x = cap.get(1)?.as_str().parse::<u32>().ok()?;
            let y = cap.get(2)?.as_str().parse::<u32>().ok()?;
            Some((x, y))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        println!("{}", result.unwrap());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        println!("{}", result.unwrap());
    }
}
