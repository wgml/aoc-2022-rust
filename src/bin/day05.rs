use std::{iter::Peekable, slice::Iter};

use aoc_2022_rust::aoc::input::lines;

type Stack = Vec<char>;

struct Operation {
    from: usize,
    to: usize,
    count: usize,
}

struct Input {
    stacks: Vec<Stack>,
    operations: Vec<Operation>,
}

fn parse(input: &Vec<String>) -> Input {
    let parse_stacks = |it: &mut Peekable<Iter<String>>| -> Vec<Stack> {
        let first = it.peek().unwrap();
        // len = 4n - 1 -> n = (len + 1) / 4
        let capacity = (first.len() + 1) / 4;

        let mut stacks: Vec<Stack> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            stacks.push(Stack::new());
        }

        for line in it {
            if !line.contains('[') {
                break;
            }

            for (i, el) in line.chars().skip(1).step_by(4).enumerate() {
                if el != ' ' {
                    stacks[i].push(el);
                }
            }
        }

        for stack in &mut stacks {
            stack.reverse();
        }

        return stacks;
    };

    let parse_operations = |it: &mut Peekable<Iter<String>>| -> Vec<Operation> {
        let mut operations = Vec::new();

        for line in it {
            let parts: Vec<&str> = line.split(' ').collect();
            let count: usize = parts[1].parse().unwrap();
            let from: usize = parts[3].parse().unwrap();
            let to: usize = parts[5].parse().unwrap();
            operations.push(Operation { from, to, count });
        }

        return operations;
    };

    let mut it = input.iter().peekable();

    let stacks = parse_stacks(&mut it);

    it.next(); // skip empty line

    let operations = parse_operations(&mut it);

    return Input { stacks, operations };
}

fn first(input: &Input) -> String {

    let mut stacks = input.stacks.clone();

    for op in &input.operations {
        for _ in 0..op.count {
            let c = stacks[op.from - 1].pop().unwrap();
            stacks[op.to - 1].push(c);
        }
    }

    return stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
}

fn second(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for op in &input.operations {
        let mut moved: Vec<char> = Vec::with_capacity(op.count);
        for _ in 0..op.count {
            moved.push(stacks[op.from - 1].pop().unwrap())
        }
        for _ in 0..op.count {
            stacks[op.to - 1].push(moved.pop().unwrap());
        }
    }

    return stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!("CMZ", first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!("MCD", second(&input));
    }

}
