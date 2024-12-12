use itertools::Itertools;
use std::ops::Div;
use std::string::String;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u128> {
    let expanded = expand(input).ok()?;
    let compacted = compact(&expanded);
    Some(checksum(&compacted))
}

pub fn part_two(input: &str) -> Option<u128> {
    let expanded = expand(input).ok()?;
    let compacted = compact_by_blocks(&expanded);
    Some(checksum(&compacted))
}

fn expand(input: &str) -> Result<Vec<Option<usize>>, String> {
    let mut result = Vec::new();
    for (i, ch) in input.chars().enumerate() {
        match ch.to_digit(10) {
            Some(count) => {
                let repeated = if i % 2 == 0 {
                    vec![Some(i.div(2)); count as usize]
                } else {
                    vec![None; count as usize]
                };
                result.extend(repeated);
            }
            None => return Err(format!("Invalid character in input: {}", ch)),
        }
    }
    Ok(result)
}

fn compact(input: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut result = input.to_vec();
    let empty_positions: Vec<_> = result.iter().positions(Option::is_none).collect();
    let mut non_empty_values: Vec<_> = input.iter().filter_map(|&ch| ch).collect();

    for &pos in empty_positions.iter() {
        result[pos] = non_empty_values.pop();
    }

    result.truncate(input.len() - empty_positions.len());
    result
}

fn compact_by_blocks(input: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut result = input.to_owned();
    let letter_chunks: Vec<_> = find_letter_chunks(&result);

    for (chunk_len, (chunk_start, chunk_end)) in letter_chunks.iter().rev() {
        let empty = find_first_empty_block(&result, *chunk_len);

        if let Some((_, (empty_start, empty_end))) = empty {
            // verify that empty block is before the chunk
            if empty_end < *chunk_start {
                move_chunk(&mut result, chunk_start, chunk_end, empty_start);
            }
        }
    }
    result
}

fn move_chunk(result: &mut [Option<usize>], src_from: &usize, src_end: &usize, dest_from: usize) {
    let chunk_len = src_end - src_from + 1;
    let slice: Vec<_> = result[*src_from..=*src_end].to_vec();

    // Clear the original chunk
    result[*src_from..=*src_end]
        .iter_mut()
        .for_each(|ch| *ch = None);

    // Move the chunk into the empty space
    result[dest_from..dest_from + chunk_len].copy_from_slice(&slice);
}

fn find_letter_chunks(input: &[Option<usize>]) -> Vec<(usize, (usize, usize))> {
    input
        .iter()
        .enumerate()
        .group_by(|&(_, ch)| ch)
        .into_iter()
        .filter_map(|(element, group)| {
            if element.is_some() {
                let indices: Vec<_> = group.map(|(index, _)| index).collect();
                let start = indices[0];
                let end = indices[indices.len() - 1];
                Some((indices.len(), (start, end)))
            } else {
                None
            }
        })
        .collect()
}

fn find_first_empty_block(
    result: &[Option<usize>],
    min_length: usize,
) -> Option<(usize, (usize, usize))> {
    result
        .iter()
        .enumerate()
        .group_by(|&(_, ch)| ch.is_none())
        .into_iter()
        .filter(|(is_none, _)| *is_none)
        .map(|(_, group)| {
            let indices: Vec<_> = group.map(|(index, _)| index).collect();
            let start = indices[0];
            let end = indices[indices.len() - 1];
            (indices.len(), (start, end))
        })
        .find(|&(empty_len, _)| empty_len >= min_length)
}

fn checksum(input: &[Option<usize>]) -> u128 {
    input.iter().enumerate().fold(0, |acc, (i, ch)| {
        acc.checked_add(ch.map_or(0, |value| (i as u128).saturating_mul(value as u128)))
            .expect("Overflow during checksum calculation")
    })
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 2858);
    }
}
