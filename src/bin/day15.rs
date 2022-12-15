use std::collections::HashSet;

use aoc_2022_rust::aoc::input::lines;

type Position = (i64, i64);
type Input = Vec<(Position, Position)>;

fn parse(input: &Vec<String>) -> Input {
    let parse_line = |line: &String| -> (Position, Position) {
        let is_separator = |c: char| -> bool {
            return match c {
                ',' | ':' | '=' => true,
                _ => false,
            };
        };
        let parts: Vec<&str> = line.split(is_separator).collect();
        let sensor_x = parts[1].parse().unwrap();
        let sensor_y = parts[3].parse().unwrap();
        let beacon_x = parts[5].parse().unwrap();
        let beacon_y = parts[7].parse().unwrap();

        return ((sensor_x, sensor_y), (beacon_x, beacon_y));
    };

    return input.iter().map(parse_line).collect();
}

fn distance(a: &Position, b: &Position) -> i64 {
    return (a.0 - b.0).abs() + (a.1 - b.1).abs();
}

fn first(input: &Input, searched_y: i64) -> usize {
    let mut not_occupied = HashSet::<Position>::new();
    let mut beacons = HashSet::<Position>::new();
    for (_, b) in input {
        beacons.insert(*b);
    }

    for (s, b) in input {
        let dist = distance(s, b);

        let (sx, sy) = s;

        for dy in -dist..=dist {
            let y = sy + dy;
            if y != searched_y {
                continue;
            }
            for dx in -dist..=dist {
                if dx.abs() + dy.abs() > dist || (dx == 0 && dy == 0) {
                    continue;
                }
                let x = sx + dx;

                if beacons.contains(&(x, y)) {
                    continue;
                }

                not_occupied.insert((x, y));
            }
        }
    }
    return not_occupied
        .iter()
        .filter(|(_, py)| *py == searched_y)
        .count();
}

fn second(input: &Input, at_most: i64) -> i64 {
    let min = 0;
    let max = at_most;

    for (sensor, beacon) in input {
        let dist = distance(sensor, beacon);

        let (sx, sy) = sensor;

        for dy in -dist..=dist {
            let y = sy + dy;
            if y < min {
                continue;
            }
            if y > max {
                break;
            }

            let dx = dist - dy.abs() + 1;
            'find: for x in [sx + dx, sx - dx] {
                if x < min || x > max {
                    continue;
                }
                for (other_sensor, other_beacon) in input {
                    if distance(other_sensor, &(x, y)) <= distance(other_sensor, other_beacon) {
                        break 'find;
                    }
                }
                return x * 4_000_000 + y;
            }
        }
    }
    return 0;
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed, 2000000));
    println!("second = {}", second(&parsed, 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(26, first(&input, 10));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(56000011, second(&input, 20));
    }
}
