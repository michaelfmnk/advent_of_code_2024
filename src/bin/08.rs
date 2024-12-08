use std::collections::HashMap;
use std::fmt::Display;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = Map::of_input(input);
    map.mark_antinodes();
    Some(map.count_antinodes())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map = Map::of_input(input);
    map.mark_antinodes_extending();
    Some(map.count_antinodes())
}

struct Node {
    signal: Option<char>,
    is_antinode: bool,
}

struct Map {
    nodes: Vec<Vec<Node>>,
}

impl Map {
    pub fn of_input(input: &str) -> Self {
        let nodes = input
            .lines()
            .map(|line| line.trim())
            .map(|line| {
                line.chars()
                    .map(|signal| Node {
                        signal: (signal != '.').then_some(signal),
                        is_antinode: false,
                    })
                    .collect()
            })
            .collect();
        Map { nodes }
    }

    pub fn mark_antinodes(&mut self) {
        self.mark_antinodes_common(false);
    }

    pub fn mark_antinodes_extending(&mut self) {
        self.mark_antinodes_common(true);
    }

    fn mark_antinodes_common(&mut self, extending: bool) {
        let mut pairs = self.antennas_combinations();

        for (_, (x1, y1), (x2, y2)) in pairs {
            if extending {
                self.nodes[y1][x1].is_antinode = true;
                self.nodes[y2][x2].is_antinode = true;

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let mut x = x1 as isize;
                let mut y = y1 as isize;
                while self.mark_antinode(x, y) {
                    x = x - dx;
                    y = y - dy;
                }

                let mut x = x2 as isize;
                let mut y = y2 as isize;
                while self.mark_antinode(x, y) {
                    x = x + dx;
                    y = y + dy;
                }
            } else {
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                self.mark_antinode(x1 as isize - dx, y1 as isize - dy);
                self.mark_antinode(x2 as isize + dx, y2 as isize + dy);
            }
        }
    }

    fn count_antinodes(&self) -> usize {
        self.nodes
            .iter()
            .map(|row| row.iter().filter(|node| node.is_antinode).count())
            .sum()
    }

    fn signal_positions(&self) -> HashMap<char, Vec<(usize, usize)>> {
        let mut signals = HashMap::new();
        for (y, row) in self.nodes.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if let Some(signal) = node.signal {
                    signals.entry(signal).or_insert_with(Vec::new).push((x, y));
                }
            }
        }
        signals
    }

    fn antennas_combinations(&self) -> Vec<(char, (usize, usize), (usize, usize))> {
        let distinct_signals: Vec<_> = self
            .signal_positions()
            .into_iter()
            .filter(|(_, positions)| positions.len() > 1)
            .collect();

        let mut pairs = Vec::new();
        for (signal, positions) in &distinct_signals {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    pairs.push((*signal, positions[i], positions[j]));
                }
            }
        }
        pairs
    }

    fn mark_antinode(&mut self, x: isize, y: isize) -> bool {
        if x >= 0 && y >= 0 && x < self.nodes[0].len() as isize && y < self.nodes.len() as isize {
            self.nodes[y as usize][x as usize].is_antinode = true;
            true
        } else {
            false
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.nodes {
            for node in row {
                if node.is_antinode {
                    write!(f, "#")?;
                } else if let Some(signal) = node.signal {
                    write!(f, "{}", signal)?;
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
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 34);
    }
}
