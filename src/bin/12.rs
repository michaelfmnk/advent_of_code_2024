advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = read_input(input);
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut result = 0;

    for (x, line) in input.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            if visited[x][y] {
                continue;
            }

            let new_visited = fill(&input, (x, y));
            result += calculate_area(&new_visited) * calculate_perimeter(&new_visited);

            merge_visited(&mut visited, new_visited);
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input = read_input(input);
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut result = 0;

    for (x, line) in input.iter().enumerate() {
        for (y, _) in line.iter().enumerate() {
            if visited[x][y] {
                continue;
            }

            let new_visited = fill(&input, (x, y));
            result += calculate_area(&new_visited) * calculate_sides(&new_visited);
            merge_visited(&mut visited, new_visited);
        }
    }

    Some(result)
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn fill(input: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<Vec<bool>> {
    let mut stack = vec![position];
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let target = input[position.0][position.1];

    while let Some((x, y)) = stack.pop() {
        if visited[x][y] || input[x][y] != target {
            continue;
        }

        visited[x][y] = true;

        if x + 1 < input.len() {
            stack.push((x + 1, y));
        }
        if x > 0 {
            stack.push((x - 1, y));
        }
        if y + 1 < input[0].len() {
            stack.push((x, y + 1));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
    }

    visited
}

fn calculate_area(visited: &[Vec<bool>]) -> usize {
    visited
        .iter()
        .map(|row| row.iter().filter(|&&cell| cell).count())
        .sum()
}

fn calculate_perimeter(visited: &[Vec<bool>]) -> usize {
    let mut perimeter = 0;

    for (x, row) in visited.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if !cell {
                continue;
            }

            if x == 0 || !visited[x - 1][y] {
                perimeter += 1;
            }
            if x + 1 == visited.len() || !visited[x + 1][y] {
                perimeter += 1;
            }
            if y == 0 || !visited[x][y - 1] {
                perimeter += 1;
            }
            if y + 1 == visited[0].len() || !visited[x][y + 1] {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_sides(visited: &[Vec<bool>]) -> usize {
    // extend visited by one cell in each direction
    let mut visited: Vec<_> = visited
        .iter()
        .map(|row| {
            let mut row = row.clone();
            row.insert(0, false);
            row.push(false);
            row
        })
        .collect();
    visited.insert(0, vec![false; visited[0].len()]);
    visited.push(vec![false; visited[0].len()]);

    let mut result = 0;

    let mut ltr = 0;
    for i in 0..visited[0].len() {
        let mut is_prev_edge = false;

        for j in 0..visited.len() {
            let is_edge =  visited[j][i] && !visited[j][i - 1];
            if is_edge && !is_prev_edge {
                ltr += 1;
            }

            is_prev_edge = is_edge;
        }
    }
    result += ltr;

    let mut rtl = 0;
    for i in (0..visited[0].len()).rev() {
        let mut is_prev_edge = false;

        for j in 0..visited.len() {
            let is_edge = visited[j][i] && !visited[j][i + 1];
            if is_edge && !is_prev_edge {
                rtl += 1;
            }

            is_prev_edge = is_edge;
        }
    }

    result += rtl;

    let mut ttb = 0;
    for i in 0..visited.len() {
        let mut is_prev_edge = false;

        for j in 0..visited[0].len() {
            let is_edge = visited[i][j] && !visited[i - 1][j];

            if is_edge && !is_prev_edge {
                ttb += 1;
            }

            is_prev_edge = is_edge;
        }
    }
    result += ttb;

    let mut btt = 0;
    for i in (0..visited.len()).rev() {
        let mut is_prev_edge = false;

        for j in 0..visited[0].len() {
            let is_edge = visited[i][j] && !visited[i + 1][j];

            if is_edge && !is_prev_edge {
                btt += 1;
            }

            is_prev_edge = is_edge;
        }
    }
    result += btt;
    result
}

fn merge_visited(visited: &mut Vec<Vec<bool>>, new_visited: Vec<Vec<bool>>) {
    for (x, row) in visited.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            *cell = *cell || new_visited[x][y];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}