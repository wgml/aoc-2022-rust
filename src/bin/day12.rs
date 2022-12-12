use std::collections::{HashMap, VecDeque};

use aoc_2022_rust::aoc::input::lines;

type Point = (usize, usize);

struct HeightMap {
    start: Point,
    finish: Point,

    width: usize,
    height: usize,
    map: Vec<u8>,
}

impl HeightMap {
    fn at(&self, (x, y): &Point) -> u8 {
        return self.map[self.width * y + x];
    }

    fn neighbours(&self, (x, y): &Point) -> Vec<Point> {
        let mut result = Vec::new();
        let h = self.at(&(*x, *y));
        if *x > 0 && self.at(&(*x - 1, *y)) <= h + 1 {
            result.push((*x - 1, *y))
        }
        if *x < self.width - 1 && self.at(&(*x + 1, *y)) <= h + 1 {
            result.push((*x + 1, *y))
        }
        if *y > 0 && self.at(&(*x, *y - 1)) <= h + 1 {
            result.push((*x, *y - 1))
        }
        if *y < self.height - 1 && self.at(&(*x, *y + 1)) <= h + 1 {
            result.push((*x, *y + 1))
        }
        return result;
    }
}

fn parse(input: &Vec<String>) -> HeightMap {
    let width = input[0].len();
    let height = input.len();
    let mut map = vec![0; width * height];

    let mut start = (0, 0);
    let mut finish = (0, 0);

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            map[i * width + j] = match c {
                'S' => {
                    start = (j, i);
                    'a'
                }
                'E' => {
                    finish = (j, i);
                    'z'
                }
                _ => c,
            } as u8
                - 'a' as u8;
        }
    }
    return HeightMap {
        start,
        finish,
        width,
        height,
        map,
    };
}

fn shortest_path(map: &HeightMap, start: &Point, finish: &Point) -> Option<usize> {
    let mut to_visit = VecDeque::new();
    let mut costs = HashMap::<Point, usize>::new();

    for neighbour in &map.neighbours(start) {
        to_visit.push_back((neighbour.clone(), 1))
    }

    while let Some((point, cost)) = to_visit.pop_front() {
        if costs.contains_key(&point) && costs[&point] <= cost {
            continue;
        }

        costs.insert(point, cost);

        for n in map.neighbours(&point) {
            to_visit.push_back((n, cost + 1))
        }
    }

    return costs.get(finish).copied();
}

fn first(map: &HeightMap) -> usize {
    match shortest_path(&map, &map.start, &map.finish) {
        Some(v) => v,
        None => panic!("Should have found something!"),
    }
}

fn second(map: &HeightMap) -> usize {
    let mut candidates = Vec::new();

    for y in 0..map.height {
        for x in 0..map.width {
            if map.at(&(x, y)) == 0 {
                candidates.push((x, y));
            }
        }
    }

    let mut best = map.width * map.height;

    for c in candidates {
        match shortest_path(&map, &c, &map.finish) {
            Some(v) => best = v,
            None => continue,
        }
    }

    return best;
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(31, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(29, second(&input));
    }
}
