use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_2022_rust::aoc::input::lines;

fn parse(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut candidate: Vec<i32> = Vec::new();
    for line in input {
        let line = line.trim();
        if line.is_empty() {
            result.push(candidate);
            candidate = Vec::new();
        } else {
            candidate.push(line.parse().unwrap())
        }
    }

    result.push(candidate);

    return result;
}

fn max_n_elves(input: &Vec<Vec<i32>>, n: usize) -> i32 {
    let mut heap = BinaryHeap::<Reverse<i32>>::new();

    for elf in input {
        heap.push(Reverse(elf.iter().sum()));
        if heap.len() > n {
            heap.pop();
        }
    }

    return heap.iter().take(n).map(|Reverse(v)| v).sum();
}

fn first(input: &Vec<Vec<i32>>) -> i32 {
    return max_n_elves(input, 1);
}

fn second(input: &Vec<Vec<i32>>) -> i32 {
    return max_n_elves(input, 3);
}

fn main() {
    let parsed = parse(lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    fn test_input() -> Vec<Vec<i32>> {
        return parse(INPUT.split('\n').map(|l| l.to_string()).collect());
    }

    #[test]
    fn test_first() {
        assert_eq!(24000, first(&test_input()));
    }

    #[test]
    fn test_second() {
        assert_eq!(45000, second(&test_input()));
    }
}
