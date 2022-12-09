use aoc_2022_rust::aoc::input::lines;

use std::cmp::max;

type Input = Vec<Vec<u32>>;

fn parse(input: &Vec<String>) -> Input {
    let mut result = Vec::new();

    for line in input {
        result.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    return result;
}

fn is_visible(input: &Input, row: usize, col: usize) -> bool {
    if row == 0 || col == 0 || row == input.len() - 1 || col == input[0].len() - 1 {
        return true;
    }

    // from the left
    if (0..col).all(|c| input[row][c] < input[row][col]) {
        return true;
    }

    // from the right
    if ((col + 1)..input[row].len()).all(|c| input[row][c] < input[row][col]) {
        return true;
    }

    // from the top
    if (0..row).all(|r| input[r][col] < input[row][col]) {
        return true;
    }

    // from the bottom
    if ((row + 1)..input.len()).all(|r| input[r][col] < input[row][col]) {
        return true;
    }

    return false;
}

fn first(input: &Input) -> usize {
    let mut visible_trees = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if is_visible(input, row, col) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

fn scenic_score(input: &Input, row: usize, col: usize) -> usize {
    let mut score = 1;

    let height = input[row as usize][col as usize];

    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut dir_score = 0;
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        while r >= 0 && r < input.len() as i32 && c >= 0 && c < input[0].len() as i32 {
            if input[r as usize][c as usize] < height {
                dir_score += 1;
                r += dr;
                c += dc;
            } else {
                dir_score += 1;
                break;
            }
        }

        score *= dir_score;
    }

    return score;
}

fn second(input: &Input) -> usize {
    let mut best_score = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            best_score = max(best_score, scenic_score(input, row, col));
        }
    }

    return best_score;
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(21, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(8, second(&input));
    }
}
