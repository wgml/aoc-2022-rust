use std::collections::VecDeque;

use aoc_2022_rust::aoc::input::lines;

type Enumerated = (usize, i64);

fn parse(lines: &Vec<String>) -> Vec<i64> {
    return lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();
}

fn shuffle(values: &Vec<i64>, key: i64, iterations: usize) -> i64 {
    let mut sequence = values
        .iter()
        .map(|v| v * key)
        .enumerate()
        .collect::<VecDeque<Enumerated>>();

    for _ in 0..iterations {
        for i in 0..sequence.len() {
            let idx = sequence
                .iter()
                .enumerate()
                .find_map(|(p, (j, _))| (i == *j).then_some(p))
                .unwrap();
            sequence.rotate_left(idx);
            let (index, number) = sequence.pop_front().unwrap();
            let shift = number.rem_euclid(sequence.len() as i64) as usize;
            sequence.rotate_left(shift);
            sequence.push_front((index, number));
        }
    }
    let zero_pos = sequence
        .iter()
        .enumerate()
        .find_map(|(p, (_, v))| (*v == 0).then_some(p))
        .unwrap();

    return [1000, 2000, 3000]
        .iter()
        .map(|i| sequence[(i + zero_pos) % sequence.len()].1)
        .sum();
}

fn first(values: &Vec<i64>) -> i64 {
    shuffle(values, 1, 1)
}

fn second(values: &Vec<i64>) -> i64 {
    shuffle(values, 811589153, 10)
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(3, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(1623178306, second(&input));
    }
}
