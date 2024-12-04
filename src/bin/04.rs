advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = convert_to_matrix(input);
    let phrase = "XMAS";

    let mut result = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            result += possible_solutions_at_location(&matrix, (x, y), phrase);
        }
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = convert_to_matrix(input);

    let mut result = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if possible_xmas_middle_point(&matrix, (x, y)) {
                let diagonal_right = || {
                    let up_left = matrix[x - 1][y - 1];
                    let down_right = matrix[x + 1][y + 1];

                    (up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M')
                };

                let diagonal_left = || {
                    let up_right = matrix[x - 1][y + 1];
                    let down_left = matrix[x + 1][y - 1];

                    (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M')
                };

                if diagonal_left() && diagonal_right() {
                    result += 1;
                }
            }
        }
    }

    Some(result as u32)
}

fn possible_xmas_middle_point(p0: &Vec<Vec<char>>, p1: (usize, usize)) -> bool {
    let (x, y) = p1;
    p0[x][y] == 'A' && x > 0 && y > 0 && x < p0.len() - 1 && y < p0[0].len() - 1
}

fn possible_solutions_at_location(
    matrix: &[Vec<char>],
    coords: (usize, usize),
    phrase: &str,
) -> usize {
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    directions
        .iter()
        .filter(|&&direction| phrase_matched_in_direction(matrix, coords, direction, phrase))
        .count()
}

fn phrase_matched_in_direction(
    matrix: &[Vec<char>],
    start_coordinates: (usize, usize),
    direction: (isize, isize),
    phrase: &str,
) -> bool {
    let (x, y) = start_coordinates;
    let (dx, dy) = direction;
    let end_coords = calculate_end_coords(direction, start_coordinates, phrase.len());
    if !is_in_bounds(matrix, end_coords) {
        return false;
    }

    phrase.chars().enumerate().all(|(i, letter)| {
        let new_x = (x as isize + dx * i as isize) as usize;
        let new_y = (y as isize + dy * i as isize) as usize;

        matrix[new_x][new_y] == letter
    })
}

fn calculate_end_coords(
    direction: (isize, isize),
    coordinates: (usize, usize),
    len: usize,
) -> (isize, isize) {
    let (x, y) = coordinates;
    let (dx, dy) = direction;
    let x_diff = dx * (len as isize - 1);
    let y_diff = dy * (len as isize - 1);

    let new_x = x as isize + x_diff;
    let new_y = y as isize + y_diff;

    (new_x, new_y)
}

fn is_in_bounds(matrix: &[Vec<char>], coordinates: (isize, isize)) -> bool {
    let (x, y) = coordinates;
    x >= 0 && x < matrix.len() as isize && y >= 0 && y < matrix[0].len() as isize
}

fn convert_to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY)).unwrap();
        // 2567
        assert_eq!(result, 2567);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
