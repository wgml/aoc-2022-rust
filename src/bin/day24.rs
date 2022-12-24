use std::collections::{HashSet, VecDeque};

use aoc_2022_rust::aoc::input::lines;

type Position = (usize, usize);
type Direction = (i32, i32);

type Tile = bool;

#[derive(Debug)]
struct Blizzard {
    initial_pos: Position,
    direction: Direction,
}

impl Blizzard {
    fn at(&self, time: usize, grid: &Vec<Vec<Tile>>) -> Position {
        let width = grid[0].len() - 2;
        let height = grid.len() - 2;

        let (x, y) = self.initial_pos;
        let (dx, dy) = self.direction;

        let tx = time % width;
        let ty = time % height;

        return (
            (((x as i32 - 1) + width as i32 + dx * tx as i32) % width as i32 + 1) as usize,
            (((y as i32 - 1) + height as i32 + dy * ty as i32) % height as i32 + 1) as usize,
        );
    }
}

#[derive(Debug)]
struct Grid {
    walls: Vec<Vec<Tile>>,
    width: usize,
    height: usize,

    blizzards_at: (Vec<HashSet<Position>>, Vec<HashSet<Position>>),
}

impl Grid {
    fn has_blizzard(&self, pos: &Position, time: usize) -> bool {
        return self.blizzards_at.0[time % (self.width - 2)].contains(pos)
            || self.blizzards_at.1[time % (self.height - 2)].contains(pos);
    }
}
fn calculate_blizzards(
    walls: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    blizzards: &Vec<Blizzard>,
) -> (Vec<HashSet<Position>>, Vec<HashSet<Position>>) {
    let mut horizontal = vec![HashSet::new(); width - 2];
    let mut vertical = vec![HashSet::new(); width - 2];

    for blizzard in blizzards {
        match blizzard.direction {
            (_, 0) => {
                for t in 0..width - 2 {
                    horizontal[t].insert(blizzard.at(t, walls));
                }
            }
            (0, _) => {
                for t in 0..height - 2 {
                    vertical[t].insert(blizzard.at(t, walls));
                }
            }
            _ => unreachable!(),
        }
    }

    return (horizontal, vertical);
}

fn parse(lines: &Vec<String>) -> Grid {
    let width = lines[0].len();
    let height = lines.len();

    let mut grid = Vec::new();

    let mut blizzards = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.char_indices() {
            row.push(c == '#');

            if c != '#' && c != '.' {
                let dir = match c {
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    '^' => (0, -1),
                    'v' => (0, 1),
                    _ => unreachable!(),
                };
                blizzards.push(Blizzard {
                    initial_pos: (x, y),
                    direction: dir,
                });
            }
        }

        grid.push(row);
    }

    let blizzards_at = calculate_blizzards(&grid, width, height, &blizzards);
    return Grid {
        walls: grid,
        width,
        height,
        blizzards_at,
    };
}

fn travel(grid: &Grid, start: &Position, end: &Position, start_time: usize) -> usize {
    let mut visited = HashSet::<(Position, usize)>::new();
    let mut frontier = VecDeque::<(Position, usize)>::new();
    frontier.push_back((*start, start_time));

    while !frontier.is_empty() {
        let (candidate, time) = frontier.pop_front().unwrap();

        if visited.contains(&(candidate, time)) {
            continue;
        }
        visited.insert((candidate, time));

        if candidate == *end {
            return time;
        }

        let (x, y) = candidate;
        for (dx, dy) in [(-1 as i32, 0 as i32), (1, 0), (0, -1), (0, 1)] {
            let cx = (x as i32 + dx) as usize;
            let cy = (y as i32 + dy) as usize;

            if cx >= grid.width || cy >= grid.height {
                continue;
            }

            if grid.walls[cy][cx] {
                continue;
            }

            if visited.contains(&((cx, cy), time + 1)) {
                continue;
            }

            if grid.has_blizzard(&(cx, cy), time + 1) {
                continue;
            }

            frontier.push_back(((cx, cy), time + 1));
        }

        if !grid.has_blizzard(&candidate, time + 1) {
            frontier.push_back((candidate, time + 1));
        }
    }
    return 0;
}

fn first(grid: &Grid) -> usize {
    let start = (1, 0);
    let end = (grid.width - 2, grid.height - 1);

    return travel(grid, &start, &end, 0);
}

fn second(grid: &Grid) -> usize {
    let start = (1, 0);
    let end = (grid.width - 2, grid.height - 1);
    let at_end = travel(grid, &start, &end, 0);
    let at_start = travel(grid, &end, &start, at_end);
    return travel(grid, &start, &end, at_start);
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(18, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(54, second(&input));
    }
}
