use std::cmp::Ordering;

use aoc_2022_rust::aoc::input::lines;

#[derive(Debug, Clone)]
enum Packet {
    Empty,
    Integer(i32),
    List(Vec<Packet>),
}

type Packets = Vec<(Packet, Packet)>;

impl Packet {
    fn parse(line: &str) -> (usize, Packet) {
        if line.is_empty() || line.as_bytes()[0] == b']' {
            return (0, Packet::Empty);
        }
        if line.as_bytes()[0].is_ascii_digit() {
            let mut i = 0;
            while line.as_bytes()[i].is_ascii_digit() {
                i += 1;
            }
            let value = line[0..i].parse::<i32>().unwrap();
            return (i, Packet::Integer(value));
        }

        let mut list: Vec<Packet> = Vec::new();
        let mut i = 1;
        while i < line.len() {
            let (j, data) = Packet::parse(&line[i..]);
            i += j;
            list.push(data);
            if line.as_bytes()[i] == b']' {
                i += 1;
                break;
            }
            assert!(line.as_bytes()[i] == b',');
            i += 1;
        }

        return (i, Packet::List(list));
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Empty, Packet::Empty) => Ordering::Equal,
            (Packet::Empty, _) => Ordering::Less,
            (_, Packet::Empty) => Ordering::Greater,
            (Packet::Integer(lhs), Packet::Integer(rhs)) => lhs.cmp(rhs),
            (Packet::Integer(lhs), Packet::List(_)) => {
                return Packet::List(Vec::from([Packet::Integer(*lhs)])).cmp(other);
            }
            (Packet::List(_), Packet::Integer(rhs)) => {
                return self.cmp(&Packet::List(Vec::from([Packet::Integer(*rhs)])));
            }
            (Packet::List(lhs), Packet::List(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    match l.cmp(r) {
                        Ordering::Equal => (),
                        o => return o,
                    }
                }
                return lhs.len().cmp(&rhs.len());
            }
        }
    }
}

fn parse(input: &Vec<String>) -> Packets {
    let mut result = Packets::new();

    let mut it = input.iter();

    loop {
        let (_, left) = Packet::parse(it.next().unwrap());
        let (_, right) = Packet::parse(it.next().unwrap());

        result.push((left, right));

        if it.next() == None {
            break;
        }
    }
    return result;
}

fn first(pairs: &Packets) -> usize {
    return pairs
        .iter()
        .enumerate()
        .filter(|(_, (lhs, rhs))| lhs <= rhs)
        .map(|(i, _)| i + 1)
        .sum();
}

fn second(pairs: &Packets) -> usize {
    let mut packets: Vec<Packet> = Vec::new();
    for (l, r) in pairs {
        packets.push(l.clone());
        packets.push(r.clone());
    }

    let p1 = Packet::List(Vec::from([Packet::Integer(2)]));
    let p2 = Packet::List(Vec::from([Packet::Integer(6)]));
    packets.push(p1.clone());
    packets.push(p2.clone());

    packets.sort();

    return packets
        .iter()
        .enumerate()
        .filter(|(_, p)| *p == &p1 || *p == &p2)
        .map(|(i, _)| i + 1)
        .fold(1, |a, b| a * b);
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(13, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(140, second(&input));
    }
}
