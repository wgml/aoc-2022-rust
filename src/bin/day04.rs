use aoc_2022_rust::aoc::input::lines;

struct Interval {
    begin: u32,
    end: u32,
}

fn parse(input: &Vec<String>) -> Vec<(Interval, Interval)> {
    let to_interval = |s: &str| -> Interval {
        let parts: Vec<&str> = s.split('-').collect();
        return Interval {
            begin: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        };
    };

    input
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(',').collect();
            return (to_interval(parts[0]), to_interval(parts[1]));
        })
        .collect()
}

fn contains((first, second): &(Interval, Interval)) -> bool {
    let contains = |a: &Interval, b: &Interval| -> bool {
        return a.begin <= b.begin && a.end >= b.end;
    };
    return contains(first, second) || contains(second, first);
}

fn overlaps((first, second): &(Interval, Interval)) -> bool {
    let overlaps = |a: &Interval, b: &Interval| -> bool {
        return a.begin <= b.begin && a.end >= b.begin;
    };
    return overlaps(first, second) || overlaps(second, first);
}

fn first(intervals: &Vec<(Interval, Interval)>) -> usize {
    return intervals.iter().filter(|i| contains(*i)).count();
}

fn second(intervals: &Vec<(Interval, Interval)>) -> usize {
    return intervals.iter().filter(|i| overlaps(*i)).count();
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_first() {
        let intervals = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(2, first(&intervals));
    }

    #[test]
    fn test_second() {
        let intervals = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(4, second(&intervals));
    }
}
