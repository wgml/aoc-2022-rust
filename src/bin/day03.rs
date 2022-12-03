use std::collections::HashSet;

use aoc_2022_rust::aoc::input::lines;

fn to_set(s: &str) -> HashSet<char> {
    return s.chars().collect();
}

fn find_duplicate(rucksack: &String) -> char {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);

    let left_elements = to_set(left);
    let right_elements = to_set(right);

    *left_elements
        .iter()
        .filter(|e| right_elements.contains(e))
        .next()
        .unwrap()
}

fn find_badge(rucksacks: &[String]) -> char {
    let left = to_set(rucksacks[0].as_str());
    let middle = to_set(rucksacks[1].as_str());
    let right = to_set(rucksacks[2].as_str());

    *left
        .iter()
        .filter(|e| middle.contains(e) && right.contains(e))
        .next()
        .unwrap()
}

fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return (item as u32) - ('a' as u32) + 1;
    } else {
        return (item as u32) - ('A' as u32) + 27;
    }
}

fn first(rucksacks: &Vec<String>) -> u32 {
    rucksacks.iter().map(find_duplicate).map(priority).sum()
}

fn second(rucksacks: &Vec<String>) -> u32 {
    rucksacks
        .chunks(3)
        .into_iter()
        .map(find_badge)
        .map(priority)
        .sum()
}

fn main() {
    let parsed = lines();
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_first() {
        let rucksacks: Vec<String> = INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(157, first(&rucksacks));
    }

    #[test]
    fn test_second() {
        let rucksacks: Vec<String> = INPUT.lines().map(|l| l.trim().to_string()).collect();
        assert_eq!(70, second(&rucksacks));
    }
}
