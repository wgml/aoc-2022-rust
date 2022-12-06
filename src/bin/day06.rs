use std::collections::HashSet;

use aoc_2022_rust::aoc::input::as_str;

fn find_unique_n(input: &str, n: usize) -> Option<usize> {
    input
        .char_indices()
        .map(move |(i, _)| (i, input.chars().skip(i).take(n).collect::<HashSet<char>>()))
        .filter(|(_, slice)| slice.len() == n)
        .take(1)
        .last()
        .map(|(i, _)| i + n)
}

fn first(input: &str) -> usize {
    find_unique_n(input, 4).unwrap()
}

fn second(input: &str) -> usize {
    find_unique_n(input, 14).unwrap()
}

fn main() {
    let parsed = as_str();
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        assert_eq!(7, first("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, first("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, first("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, first("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, first("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_second() {
        assert_eq!(19, second("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, second("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, second("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, second("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, second("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
