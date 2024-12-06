#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Vector {
    dx: isize,
    dy: isize,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector { dx: 0, dy: -1 },
            Direction::Down => Vector { dx: 0, dy: 1 },
            Direction::Left => Vector { dx: -1, dy: 0 },
            Direction::Right => Vector { dx: 1, dy: 0 },
        }
    }

    pub fn apply_to_position(&self, (x, y): (usize, usize)) -> (isize, isize) {
        let Vector { dx, dy } = self.to_vector();
        (x as isize + dx, y as isize + dy)
    }

    pub fn of_symbol(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn to_symbol(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}
