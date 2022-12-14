use std::collections::HashMap;

use aoc_2022_rust::aoc::input::lines;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Material {
    Air,
    Sand,
    Rock,
}

type Position = (i32, i32);
type Cave = HashMap<Position, Material>;

fn parse(input: &Vec<String>) -> Cave {
    let parse_pos = |s: &str| -> Position {
        let (x, y) = s.split_once(',').unwrap();
        return (x.parse().unwrap(), y.parse().unwrap());
    };

    let mut cave = Cave::new();

    for path in input {
        let points: Vec<Position> = path.split(" -> ").map(|p| parse_pos(p)).collect();

        let (mut cx, mut cy) = points[0];
        cave.insert((cx, cy), Material::Rock);

        for (px, py) in &points[1..] {
            if cx == *px {
                for y in cy..=*py {
                    cave.insert((cx, y), Material::Rock);
                }
                for y in *py..=cy {
                    cave.insert((cx, y), Material::Rock);
                }
            } else {
                for x in cx..=*px {
                    cave.insert((x, cy), Material::Rock);
                }
                for x in *px..=cx {
                    cave.insert((x, cy), Material::Rock);
                }
            }
            cx = *px;
            cy = *py;
        }
    }
    return cave;
}

fn sand_simulator(c: &Cave, max_depth: i32, occupied: &dyn Fn(&Position, &Cave) -> bool) -> usize {
    let source: Position = (500, 0);
    let mut cave = c.clone();

    while !cave.contains_key(&source) {
        let (mut sx, mut sy) = source;
        while sy <= max_depth {
            if !occupied(&(sx, sy + 1), &cave) {
                sy += 1;
            } else if !occupied(&(sx - 1, sy + 1), &cave) {
                sy += 1;
                sx -= 1;
            } else if !occupied(&(sx + 1, sy + 1), &cave) {
                sy += 1;
                sx += 1;
            } else {
                cave.insert((sx, sy), Material::Sand);
                break;
            }
        }
        if sy > max_depth {
            break;
        }
    }
    return cave.iter().filter(|(_, m)| **m == Material::Sand).count();
}

fn first(c: &Cave) -> usize {
    let max_depth = *c.iter().map(|((_, py), _)| py).max().unwrap();
    let occupied = |position: &Position, cave: &Cave| -> bool {
        cave.contains_key(position) && cave[position] != Material::Air
    };

    return sand_simulator(&c, max_depth, &occupied);
}

fn second(c: &Cave) -> usize {
    let max_depth = *c.iter().map(|((_, py), _)| py).max().unwrap() + 2;

    let occupied = |position: &Position, cave: &Cave| -> bool {
        position.1 == max_depth || cave.contains_key(position) && cave[position] != Material::Air
    };

    return sand_simulator(&c, max_depth, &occupied);
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(24, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(93, second(&input));
    }
}
