use std::collections::HashSet;

use aoc_2022_rust::aoc::input::lines;

struct Instruction {
    dx: i32,
    dy: i32,
    count: usize,
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

type Input = Vec<Instruction>;

fn parse(input: &Vec<String>) -> Input {
    let mut result = Vec::new();

    for line in input {
        let (dir, count) = line.split_once(' ').unwrap();
        match dir {
            "U" => result.push(Instruction {
                dx: 0,
                dy: -1,
                count: count.parse().unwrap(),
            }),
            "D" => result.push(Instruction {
                dx: 0,
                dy: 1,
                count: count.parse().unwrap(),
            }),
            "L" => result.push(Instruction {
                dx: -1,
                dy: 0,
                count: count.parse().unwrap(),
            }),
            "R" => result.push(Instruction {
                dx: 1,
                dy: 0,
                count: count.parse().unwrap(),
            }),
            _ => panic!("unhandled {}", dir),
        }
    }

    return result;
}

fn maybe_move(head: &Position, tail: &Position) -> Position {
    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        return tail.clone();
    }

    if head.x == tail.x || head.y == tail.y {
        if head.x == tail.x {
            return Position {
                x: tail.x,
                y: (head.y + tail.y) / 2,
            };
        } else {
            return Position {
                x: (head.x + tail.x) / 2,
                y: tail.y,
            };
        }
    }

    let mut x = (head.x + tail.x) / 2;
    if x == tail.x {
        x = head.x
    }

    let mut y = (head.y + tail.y) / 2;
    if y == tail.y {
        y = head.y
    }

    return Position { x, y };
}

fn move_rope_of_length(instructions: &Input, length: usize) -> usize {
    let mut rope = vec![Position { x: 0, y: 0 }; length];

    let mut visited_by_tail = HashSet::<(i32, i32)>::new();
    visited_by_tail.insert((0, 0));

    for instr in instructions {
        for _ in 0..instr.count {
            rope[0].x += instr.dx;
            rope[0].y += instr.dy;

            for i in 1..length {
                rope[i] = maybe_move(&rope[i - 1], &rope[i]);
            }

            let tail = rope.last().unwrap();
            visited_by_tail.insert((tail.x, tail.y));
        }
    }

    return visited_by_tail.len();
}

fn first(instructions: &Input) -> usize {
    return move_rope_of_length(instructions, 2);
}

fn second(instructions: &Input) -> usize {
    return move_rope_of_length(instructions, 10);
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_first() {
        let input = parse(&INPUT1.lines().map(|l| l.to_string()).collect());
        assert_eq!(13, first(&input));
    }

    #[test]
    fn test_second() {
        let input1 = parse(&INPUT1.lines().map(|l| l.to_string()).collect());
        assert_eq!(1, second(&input1));
        let input2 = parse(&INPUT2.lines().map(|l| l.to_string()).collect());
        assert_eq!(36, second(&input2));
    }
}
