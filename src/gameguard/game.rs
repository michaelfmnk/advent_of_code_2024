use crate::gameguard::direction;
use std::fmt::Display;

pub struct Game {
    pub map: Vec<Vec<char>>,
    pub position: (usize, usize),
    pub direction: direction::Direction,
}

impl Game {
    const VISITED: char = 'X';
    const EMPTY: char = '.';
    const BARRIER: char = '#';

    pub fn move_forward(&mut self) -> bool {
        let new_position = self.direction.apply_to_position(self.position);
        if self.is_outside(&new_position) {
            self.mark_visited();
            return false;
        }

        let new_position = (new_position.0 as usize, new_position.1 as usize);
        if self.is_barrier(&new_position) {
            self.direction = self.direction.turn_right();
            self.mark_pawn();
            return true;
        }

        self.mark_pawn();
        self.mark_visited();
        self.position = new_position;
        true
    }

    pub fn count_visited(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|&&cell| cell == Self::VISITED)
            .count()
    }

    fn mark_visited(&mut self) {
        let (x, y) = self.position;
        self.map[y][x] = Self::VISITED;
    }

    fn mark_pawn(&mut self) {
        let (x, y) = self.position;
        self.map[y][x] = self.direction.to_symbol();
    }

    fn is_outside(&self, &(x, y): &(isize, isize)) -> bool {
        x < 0 || y < 0 || y >= self.map.len() as isize || x >= self.map[0].len() as isize
    }

    fn is_barrier(&self, &(x, y): &(usize, usize)) -> bool {
        self.map[y][x] == Self::BARRIER
    }

    pub fn set_barrier_at(&mut self, (x, y): (usize, usize)) {
        self.map[y][x] = Self::BARRIER;
    }

    pub fn is_empty(&self, (x, y): (usize, usize)) -> bool {
        self.map[y][x] == Self::EMPTY
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Clone for Game {
    fn clone(&self) -> Game {
        Game {
            map: self.map.clone(),
            position: self.position.clone(),
            direction: self.direction.clone(),
        }
    }
}
