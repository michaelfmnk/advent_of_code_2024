use advent_of_code::gameguard::direction::Direction;
use advent_of_code::gameguard::game::Game;
use std::fmt::Display;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut game = initialize_game(input);
    while game.move_forward() {}
    Some(game.count_visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game = initialize_game(input);
    let mut counter = 0;

    for (y, row) in game.map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if game.is_empty((x, y)) {
                let mut game = game.clone();
                game.set_barrier_at((x, y));

                let mut loop_limit = 4973 * 5;
                while game.move_forward() && loop_limit > 0 {
                    loop_limit -= 1;
                }

                if loop_limit == 0 {
                    counter += 1;
                }
            }
        }
    }

    Some(counter as u32)
}

fn initialize_game(input: &str) -> Game {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let (position, direction) = find_pawn(&map).expect("Pawn not found");

    Game {
        map,
        position,
        direction,
    }
}

fn find_pawn(map: &Vec<Vec<char>>) -> Option<((usize, usize), Direction)> {
    map.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| {
                if cell != '.' && cell != '#' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .map(|(x, y)| {
            let pawn_symbol = map[y][x];
            let direction = Direction::of_symbol(pawn_symbol);
            ((x, y), direction)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 6);
    }
}
