use std::fmt::{Display, Formatter};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = Map::new(input, (101, 103));
    println!("{}", map);

    for i in 0..100 {
        map = map.move_guards();
    }

    let (q1, q2, q3, q4) = map.count_quadrants();

    Some(q1 * q2 * q3 * q4)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Clone)]
struct Guard {
    vx: i32,
    vy: i32,
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Vec<Guard>>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for col in row {
                if col.is_empty() {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", col.len())?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new(input: &str, (x, y): (usize, usize)) -> Self {
        let mut result = Self {
            tiles: vec![vec![vec![]; x]; y],
            width: x,
            height: y,
        };
        result.parse(input);
        result
    }

    pub fn count_quadrants(&self) -> (usize, usize, usize, usize) {
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        let left_q_end = (self.width as f64 / 2.0).floor() as usize; // 4/2 = 2; floor = 2 [0, 1] || 5/2 = 2.5; floor = 2 [0, 1] - exclusive
        let right_q_start = (self.width as f64 / 2.0).ceil() as usize; // 4/2 = 2; ceil = 2 [2, 3] || 5/2 = 2.5; ceil = 3 [3, 4] - inclusive

        let top_q_end = (self.height as f64 / 2.0).floor() as usize; //
        let bottom_q_start = (self.height as f64 / 2.0).ceil() as usize;

        for y in 0..self.height {
            for x in 0..self.width {
                if x < left_q_end && y < top_q_end {
                    q1 += self.tiles[y][x].len();
                } else if x >= right_q_start && y < top_q_end {
                    q2 += self.tiles[y][x].len();
                } else if x < left_q_end && y >= bottom_q_start {
                    q3 += self.tiles[y][x].len();
                } else if x >= right_q_start && y >= bottom_q_start {
                    q4 += self.tiles[y][x].len();
                }
            }
        }

        (q1, q2, q3, q4)
    }

    pub fn move_guards(&self) -> Map {
        let mut new_map = self.clone();
        new_map.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                for guard in &self.tiles[y][x] {
                    let (new_x, new_y) = self.calculate_coords_warping(x, y, guard.vx, guard.vy);
                    new_map.tiles[new_y][new_x].push(Guard {
                        vx: guard.vx,
                        vy: guard.vy,
                    });
                }
            }
        }
        new_map
    }

    fn clear(&mut self) {
        for row in &mut self.tiles {
            for col in row {
                col.clear();
            }
        }
    }

    fn calculate_coords_warping(&self, x: usize, y: usize, vx: i32, vy: i32) -> (usize, usize) {
        let new_x = (x as i32 + vx).rem_euclid(self.width as i32);
        let new_y = (y as i32 + vy).rem_euclid(self.height as i32);
        // 1 - 3 =

        (new_x as usize, new_y as usize)
    }

    fn parse(&mut self, input: &str) {
        let re = regex::Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
        for line in input.lines() {
            let caps = re.captures(line).unwrap();
            let x = caps[1].parse::<usize>().unwrap();
            let y = caps[2].parse::<usize>().unwrap();
            let vx = caps[3].parse::<i32>().unwrap();
            let vy = caps[4].parse::<i32>().unwrap();
            self.tiles[y][x].push(Guard { vx, vy });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        println!("{}", -3 % 11);
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
