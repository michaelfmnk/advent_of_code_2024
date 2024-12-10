use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{self, Display};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let reachability = compute_reachability_map(input);

    Some(reachability
        .values()
        .map(|positions| positions.iter().unique().count())
        .sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let reachability = compute_reachability_map(input);
    Some(reachability.values().map(Vec::len).sum())
}

fn compute_reachability_map(input: &str) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut map = Map::new(input);
    let mut positions_to_explore = map
        .find_positions_with_value(0)
        .into_iter()
        .map(|pos| (pos, pos))
        .collect_vec();

    let mut reachability: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    while let Some((start_pos, current_pos)) = positions_to_explore.pop() {
        map.set_position(Some(current_pos));

        while !map.is_at_top() {
            let next_positions = map.find_next_positions();

            if next_positions.len() == 1 {
                map.set_position(Some(next_positions[0]));
            } else {
                positions_to_explore.extend(next_positions.into_iter().map(|pos| (start_pos, pos)));
                break;
            }
        }

        if map.is_at_top() {
            reachability
                .entry(map.current_position().unwrap())
                .or_default()
                .push(start_pos);
        }
    }
    reachability
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Option<usize>>>,
    current_position: Option<(usize, usize)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).map(|n| n as usize))
                    .collect()
            })
            .collect();

        Self {
            grid,
            current_position: None,
        }
    }

    fn set_position(&mut self, position: Option<(usize, usize)>) {
        self.current_position = position;
    }

    fn current_position(&self) -> Option<(usize, usize)> {
        self.current_position
    }

    fn find_positions_with_value(&self, value: usize) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &cell)| {
                    if cell == Some(value) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn find_next_positions(&self) -> Vec<(usize, usize)> {
        if let Some((x, y)) = self.current_position {
            if let Some(value) = self.grid[y][x] {
                self.find_adjacent_positions_with_value(value + 1)
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    fn find_adjacent_positions_with_value(&self, value: usize) -> Vec<(usize, usize)> {
        let (x, y) = self.current_position.unwrap_or((0, 0));
        let mut positions = Vec::new();

        let directions = [
            (0, 1),
            (1, 0),
            (0, usize::MAX), // Simulates wrapping subtract for y > 0
            (usize::MAX, 0), // Simulates wrapping subtract for x > 0
        ];

        for &(dx, dy) in &directions {
            let new_x = x.wrapping_add(dx);
            let new_y = y.wrapping_add(dy);

            if new_y < self.grid.len()
                && new_x < self.grid[new_y].len()
                && self.grid[new_y][new_x] == Some(value)
            {
                positions.push((new_x, new_y));
            }
        }
        positions
    }

    fn is_at_top(&self) -> bool {
        matches!(self.current_position, Some((x, y)) if self.grid[y][x] == Some(9))
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &cell in row {
                if let Some(value) = cell {
                    write!(f, "{}", value)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 81);
    }
}
