use std::fmt::Display;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let mut simulation = Simulation::of(input);

    while !simulation.finished() {
        simulation.move_robot();
    }

    Some(simulation.boxes_gps_score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut simulation = Simulation::of(input);
    simulation.widen();

    while !simulation.finished() {
        simulation.move_robot();
    }

    Some(simulation.boxes_gps_score())
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Object {
    BoxR,
    BoxL,
    Box,
    Robot,
    Wall,
}

#[derive(Debug)]
struct Simulation {
    matrix: Vec<Vec<Option<Object>>>,
    robot: (usize, usize),
    directions: Vec<Direction>,
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, object) in row.iter().enumerate() {
                if self.robot == (x, y) {
                    write!(f, "@")?;
                } else {
                    match object {
                        Some(Object::Robot) => write!(f, "@")?,
                        Some(Object::Box) => write!(f, "O")?,
                        Some(Object::Wall) => write!(f, "#")?,
                        Some(Object::BoxL) => write!(f, "[")?,
                        Some(Object::BoxR) => write!(f, "]")?,
                        None => write!(f, ".")?,
                        o => panic!("Invalid object: {:?}", o),
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Simulation {
    fn of(input: &str) -> Self {
        let parts: Vec<_> = input.split("\n\n").collect();
        let matrix = parts[0];
        let directions = parts[1];

        let matrix: Vec<_> = matrix
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Some(Object::Wall),
                        'O' => Some(Object::Box),
                        '@' => Some(Object::Robot),
                        '.' => None,
                        _ => panic!("Invalid character"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut directions = directions
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| match c {
                '^' => Direction::North,
                'v' => Direction::South,
                '>' => Direction::East,
                '<' => Direction::West,
                _ => panic!("Invalid character"),
            })
            .rev()
            .collect();

        let robot = matrix
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, object)| match object {
                    Some(Object::Robot) => Some((x, y)),
                    _ => None,
                })
            })
            .unwrap();

        Self {
            matrix,
            directions,
            robot,
        }
    }

    fn widen(&mut self) {
        let matrix = self.matrix.clone();

        for (y, row) in matrix.iter().enumerate() {
            for (x, object) in row.iter().enumerate().rev() {
                if x == 3 && y == 4 {
                    true;
                }
                if let Some(Object::Robot) = object {
                    self.matrix[y].insert(x + 1, None);
                } else if let Some(Object::Box) = object {
                    self.matrix[y][x] = Some(Object::BoxL);
                    self.matrix[y].insert(x + 1, Some(Object::BoxR));
                } else if let Some(Object::Wall) = object {
                    self.matrix[y].insert(x + 1, Some(Object::Wall));
                } else {
                    self.matrix[y].insert(x + 1, None);
                }
            }
        }

        self.robot = (self.robot.0 * 2, self.robot.1);
    }

    fn finished(&self) -> bool {
        self.directions.is_empty()
    }

    fn move_robot(&mut self) {
        let direction = self.directions.pop().unwrap();
        let (x, y) = self.robot;
        assert_eq!(self.matrix[y][x], Some(Object::Robot));

        self.robot = self.move_object_at((x, y), &direction);
        assert_eq!(self.matrix[self.robot.1][self.robot.0], Some(Object::Robot));
    }

    fn move_object_at(&mut self, (x, y): (usize, usize), direction: &Direction) -> (usize, usize) {
        let (new_x, new_y) = direction.apply((x, y));

        let obstacle = &self.matrix[new_y][new_x];
        match obstacle {
            Some(Object::Wall) => (x, y),
            Some(Object::Box) => {
                let new_box_pos = self.move_object_at((new_x, new_y), direction);
                if new_box_pos == (new_x, new_y) {
                    // box can't move
                    (x, y)
                } else {
                    let object = self.matrix[y][x].take();
                    self.matrix[new_box_pos.1][new_box_pos.0] = Some(Object::Box);
                    self.matrix[new_y][new_x] = object;
                    (new_x, new_y)
                }
            }
            Some(Object::BoxR) => {
                let box_r_pos = (new_x, new_y);
                let box_l_pos = (new_x - 1, new_y);

                let can_move_r = self.can_move(box_r_pos, direction);
                let can_move_l = self.can_move(box_l_pos, direction);

                if can_move_l && can_move_r {
                    let object = self.matrix[y][x].take();
                    let (new_box_l_pos, new_box_r_pos) = if *direction == Direction::East {
                        let new_box_r_pos = self.move_object_at(box_r_pos, direction);
                        let new_box_l_pos = self.move_object_at(box_l_pos, direction);
                        (new_box_l_pos, new_box_r_pos)
                    } else {
                        let new_box_l_pos = self.move_object_at(box_l_pos, direction);
                        let new_box_r_pos = self.move_object_at(box_r_pos, direction);
                        (new_box_l_pos, new_box_r_pos)
                    };

                    self.matrix[new_box_r_pos.1][new_box_r_pos.0] = Some(Object::BoxR);
                    self.matrix[new_box_l_pos.1][new_box_l_pos.0] = Some(Object::BoxL);

                    self.matrix[new_y][new_x] = object;
                    self.matrix[y][x] = None;
                    (new_x, new_y)
                } else {
                    (x, y)
                }
            }
            Some(Object::BoxL) => {
                let box_r_pos = (new_x + 1, new_y);
                let box_l_pos = (new_x, new_y);

                let can_move_r = self.can_move(box_r_pos, direction);
                let can_move_l = self.can_move(box_l_pos, direction);

                if can_move_l && can_move_r {
                    let object = self.matrix[y][x].take();
                    let new_box_r_pos = self.move_object_at(box_r_pos, direction);
                    let new_box_l_pos = self.move_object_at(box_l_pos, direction);

                    self.matrix[new_box_r_pos.1][new_box_r_pos.0] = Some(Object::BoxR);
                    self.matrix[new_box_l_pos.1][new_box_l_pos.0] = Some(Object::BoxL);

                    self.matrix[new_y][new_x] = object;
                    self.matrix[y][x] = None;
                    (new_x, new_y)
                } else {
                    (x, y)
                }
            }
            _ => {
                self.matrix[y][x] = None;
                self.matrix[new_y][new_x] = Some(Object::Robot);
                (new_x, new_y)
            }
        }
    }

    fn can_move(&self, (x, y): (usize, usize), direction: &Direction) -> bool {
        let (new_x, new_y) = direction.apply((x, y));

        let obstacle = &self.matrix[new_y][new_x];
        match obstacle {
            Some(Object::Wall) => false,
            Some(Object::Box) => self.can_move((new_x, new_y), direction),
            Some(Object::BoxR) => {
                if *direction == Direction::East {
                    self.can_move((new_x, new_y), direction)
                } else if *direction == Direction::West {
                    let box_l_pos = (new_x - 1, new_y);
                    self.can_move(box_l_pos, direction)
                } else {
                    let box_r_pos = (new_x, new_y);
                    let box_l_pos = (new_x - 1, new_y);

                    let can_move_r = self.can_move(box_r_pos, direction);
                    let can_move_l = self.can_move(box_l_pos, direction);

                    can_move_l && can_move_r
                }
            }
            Some(Object::BoxL) => {
                if *direction == Direction::East {
                    let box_r_pos = (new_x + 1, new_y);
                    self.can_move(box_r_pos, direction)
                } else if *direction == Direction::West {
                    self.can_move((new_x, new_y), direction)
                } else {
                    let box_r_pos = (new_x + 1, new_y);
                    let box_l_pos = (new_x, new_y);

                    let can_move_r = self.can_move(box_r_pos, direction);
                    let can_move_l = self.can_move(box_l_pos, direction);

                    can_move_l && can_move_r
                }
            }
            None => true,
            _ => false,
        }
    }

    fn boxes_gps_score(&self) -> usize {
        self.find_all_boxes()
            .iter()
            .map(|&(x, y)| y * 100 + x)
            .sum()
    }

    fn find_all_boxes(&self) -> Vec<(usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, object)| match object {
                        Some(Object::Box) | Some(Object::BoxL) => Some((x, y)),
                        _ => None,
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 9021);
    }
}
