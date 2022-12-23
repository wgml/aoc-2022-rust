use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2022_rust::aoc::input::lines;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    None,
    Open,
    Wall,
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            ' ' => Tile::None,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("Bad tile: {}", c),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Input {
    grid: Vec<Vec<Tile>>,
    instructions: Vec<Instruction>,
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut str = String::new();
    let mut result = Vec::new();

    for c in line.chars() {
        if c == 'L' || c == 'R' {
            if !str.is_empty() {
                result.push(Instruction::Move(str.parse().unwrap()));
                str.clear();
            }
            if c == 'L' {
                result.push(Instruction::Left);
            } else {
                result.push(Instruction::Right);
            }
        } else {
            str.push(c);
        }
    }

    if !str.is_empty() {
        result.push(Instruction::Move(str.parse().unwrap()));
    }
    return result;
}

fn parse(lines: &Vec<String>) -> Input {
    let mut grid = Vec::new();

    let width = lines.iter().map(|l: &String| l.len()).max().unwrap() + 2;

    grid.push(vec![Tile::None; width]);
    for line in lines.iter().take_while(|l: &&String| !l.is_empty()) {
        grid.push(
            [
                vec![Tile::None; 1],
                line.chars().map(|c| Tile::from(c)).collect(),
                vec![Tile::None; width - line.len() - 1],
            ]
            .concat(),
        );
    }
    grid.push(vec![Tile::None; width]);

    let instructions = parse_instructions(lines.last().unwrap());

    return Input { grid, instructions };
}

fn first(input: &Input) -> usize {
    let grid = &input.grid;

    let mut y = 1 as usize;
    let mut x = input.grid[y].iter().position(|t| *t != Tile::None).unwrap();

    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction: usize = 0;
    for instr in &input.instructions {
        match instr {
            Instruction::Left => direction = (direction + 4 - 1) % 4,
            Instruction::Right => direction = (direction + 4 + 1) % 4,
            Instruction::Move(distance) => {
                let (dx, dy) = directions[direction];
                for _ in 0..*distance {
                    let mut cx = (x as i32 + dx) as usize;
                    let mut cy = (y as i32 + dy) as usize;
                    if grid[cy][cx] == Tile::None {
                        loop {
                            let nx = (cx as i32 - dx) as usize;
                            let ny = (cy as i32 - dy) as usize;

                            if grid[ny][nx] == Tile::None {
                                break;
                            }
                            cx = nx;
                            cy = ny;
                        }
                    }
                    if grid[cy][cx] == Tile::Wall {
                        break;
                    }
                    x = cx;
                    y = cy;
                }
            }
        }
    }

    return 1000 * y + 4 * x + direction;
}

fn wrap((x, y): (usize, usize), (dx, dy): (i32, i32)) -> ((usize, usize), (i32, i32)) {
    match (dx, dy, x / 50, y / 50) {
        (_, 1, _, 0) => ((99, 149 - y), (0, -1)), //return complex(149-x, 99), -1j
        (_, 1, _, 1) => ((y + 50, 49), (-1, 0)),  //return complex( 49,x+ 50), -1
        (_, 1, _, 2) => ((149, 149 - y), (0, -1)), //return complex(149-x,149), -1j
        (_, 1, _, 3) => ((y - 100, 149), (-1, 0)), //return complex(149,x-100), -1
        (_, -1, _, 0) => ((0, 149 - y), (0, 1)),  //return complex(149-x,  0),  1j
        (_, -1, _, 1) => ((y - 50, 100), (1, 0)), //return complex(100,x- 50),  1
        (_, -1, _, 2) => ((50, 149 - y), (0, 1)), //return complex(149-x, 50),  1j
        (_, -1, _, 3) => ((y - 100, 0), (1, 0)),  //return complex(  0,x-100),  1
        (1, _, 0, _) => ((x + 100, 0), (1, 0)),   //return complex(  0,y+100),  1
        (1, _, 1, _) => ((49, 100 + x), (0, -1)), //return complex(100+y, 49), -1j
        (1, _, 2, _) => ((99, x - 50), (0, -1)),  //return complex(-50+y, 99), -1j
        (-1, _, 0, _) => ((50, 50 + x), (0, 1)),  //return complex( 50+y, 50),  1j
        (-1, _, 1, _) => ((0, 100 + x), (0, 1)),  //return complex(100+y,  0),  1j
        (-1, _, 2, _) => ((x - 100, 199), (-1, 0)), //return complex(199,y-100), -1
        _ => unreachable!(),
    }
}

fn second(input: &Input) -> usize {
    let grid = &input.grid;

    let mut y = 1 as usize;
    let mut x = input.grid[y].iter().position(|t| *t != Tile::None).unwrap();

    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction: usize = 0;
    for instr in &input.instructions {
        match instr {
            Instruction::Left => direction = (direction + 4 - 1) % 4,
            Instruction::Right => direction = (direction + 4 + 1) % 4,
            Instruction::Move(distance) => {
                let (mut dx, mut dy) = directions[direction];
                for _ in 0..*distance {
                    let mut cx = (x as i32 + dx) as usize;
                    let mut cy = (y as i32 + dy) as usize;
                    if grid[cy][cx] == Tile::None {
                        println!("pre {:?},{:?}", (x - 1, y - 1), (dx, dy));
                        ((cx, cy), (dx, dy)) = wrap((x - 1, y - 1), (dx, dy));
                        cx = (cx as i32 - dx) as usize;
                        cy = (cy as i32 - dy) as usize;
                        println!("post {:?},{:?}", (cx, cy), (dx, dy));
                    }
                    if grid[cy][cx] == Tile::Wall {
                        break;
                    }
                    x = cx;
                    y = cy;
                }
            }
        }
    }

    return 1000 * y + 4 * x + direction;
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(6032, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(5031, second(&input));
    }
}
