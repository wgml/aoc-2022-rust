use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2022_rust::aoc::input::lines;

type Position = (i32, i32);

type Grid = HashSet<Position>;

enum Direction {
    North,
    South,
    West,
    East,
}

fn parse(lines: &Vec<String>) -> Grid {
    let mut grid = Grid::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, p) in line.char_indices() {
            if p == '#' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    return grid;
}

fn has_neighbours(grid: &Grid, position: &Position) -> bool {
    for x in position.0 - 1..=position.0 + 1 {
        for y in position.1 - 1..=position.1 + 1 {
            if (x, y) != *position && grid.contains(&(x, y)) {
                return true;
            }
        }
    }
    return false;
}

fn simulate_round(grid: &Grid, move_order: &VecDeque<Direction>) -> (Grid, bool) {
    let contains_any = |positions: [Position; 3]| -> bool {
        return grid.contains(&positions[0])
            || grid.contains(&positions[1])
            || grid.contains(&positions[2]);
    };

    let mut new_grid = Grid::new();

    let mut moves = HashMap::<Position, Vec<Position>>::new();

    let mut add_move = |from: Position, to: Position| {
        if !moves.contains_key(&to) {
            moves.insert(to, Vec::new());
        }
        moves.get_mut(&to).unwrap().push(from);
    };

    for elf in grid {
        if !has_neighbours(grid, elf) {
            new_grid.insert(*elf);
            continue;
        }

        let (x, y) = *elf;

        let mut moved = false;
        for dir in move_order {
            if moved {
                break;
            }

            match dir {
                Direction::North => {
                    if !contains_any([(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]) {
                        add_move((x, y), (x, y - 1));
                        moved = true;
                    }
                }
                Direction::South => {
                    if !contains_any([(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]) {
                        add_move((x, y), (x, y + 1));
                        moved = true;
                    }
                }
                Direction::West => {
                    if !contains_any([(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]) {
                        add_move((x, y), (x - 1, y));
                        moved = true;
                    }
                }
                Direction::East => {
                    if !contains_any([(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]) {
                        add_move((x, y), (x + 1, y));
                        moved = true;
                    }
                }
            }
        }
        if !moved {
            new_grid.insert(*elf);
        }
    }

    let mut any_moved = false;
    for (pos, candidates) in &moves {
        if candidates.len() == 1 {
            new_grid.insert(*pos);
            any_moved = true;
        } else {
            new_grid.extend(candidates);
        }
    }

    return (new_grid, any_moved);
}

fn simulate_rounds(grid: &Grid, rounds: usize) -> (Grid, usize) {
    let mut our_grid = grid.clone();

    let mut move_order = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    for r in 0..rounds {
        let (new_grid, any_moved) = simulate_round(&our_grid, &move_order);

        if !any_moved {
            return (new_grid, r + 1);
        }

        our_grid = new_grid;

        let m = move_order.pop_front().unwrap();
        move_order.push_back(m);
    }

    return (our_grid, rounds);
}

fn first(grid: &Grid) -> i32 {
    let (our_grid, _) = simulate_rounds(&grid, 10);

    let min_x = our_grid.iter().map(|(x, _)| x).min().unwrap();
    let max_x = our_grid.iter().map(|(x, _)| x).max().unwrap();
    let min_y = our_grid.iter().map(|(_, y)| y).min().unwrap();
    let max_y = our_grid.iter().map(|(_, y)| y).max().unwrap();

    return (max_y - min_y + 1) * (max_x - min_x + 1) - our_grid.len() as i32;
}

fn second(grid: &Grid) -> usize {
    let (_, rounds) = simulate_rounds(&grid, usize::MAX);

    return rounds;
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(110, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(20, second(&input));
    }
}
