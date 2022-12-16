use std::collections::HashMap;

use aoc_2022_rust::aoc::input::lines;

#[derive(Clone)]
struct Position {
    valve: String,
    flow: i32,
    tunnels: Vec<String>,
}

type Scan = Vec<Position>;

fn parse(input: &Vec<String>) -> Scan {
    let parse_line = |line: &String| -> Position {
        let is_separator = |c: char| -> bool {
            return match c {
                ' ' | ',' | ';' | '=' => true,
                _ => false,
            };
        };
        let parts: Vec<&str> = line.split(is_separator).collect();
        let valve = parts[1].to_string();
        let flow = parts[5].parse().unwrap();
        let tunnels = parts[11..].chunks(2).map(|c| c[0].to_string()).collect();

        return Position {
            valve,
            flow,
            tunnels,
        };
    };

    return input.iter().map(parse_line).collect();
}

fn calculate(mut scan: Scan) -> (Vec<Vec<Vec<i32>>>, usize, usize) {
    scan.sort_by(|lhs, rhs| rhs.flow.cmp(&lhs.flow));

    let valve_indexes = scan
        .iter()
        .enumerate()
        .map(|(i, v)| (&v.valve, i))
        .collect::<HashMap<&String, usize>>();

    let start_pos = valve_indexes[&"AA".to_string()];

    let unclogged_count = scan.iter().filter(|v| v.flow > 0).count();
    let total_count = valve_indexes.len();

    let mut paths = vec![vec![0; 0]; total_count];
    let mut flows = vec![0; total_count];

    for p in &scan {
        let index = valve_indexes[&p.valve];
        flows[index] = p.flow;
        for tunnel in &p.tunnels {
            paths[index].push(valve_indexes[&tunnel]);
        }
    }

    let valves_space_size = 1 << unclogged_count;
    // time left x node x unused valves bitset
    let mut dp = vec![vec![vec![0; valves_space_size]; total_count]; 30];

    for time_left in 1..30 {
        for valve in 0..total_count {
            for combination in 0..valves_space_size {
                let mut candidate = dp[time_left][valve][combination];
                if (1 << valve) & combination != 0 && time_left > 1 {
                    candidate = candidate.max(
                        dp[time_left - 1][valve][combination - (1 << valve)]
                            + flows[valve] * time_left as i32,
                    );
                }

                for path in &paths[valve] {
                    candidate = candidate.max(dp[time_left - 1][*path][combination]);
                }

                dp[time_left][valve][combination] = candidate;
            }
        }
    }
    return (dp, start_pos, valves_space_size);
}

fn first(scan: Scan) -> i32 {
    let (dp, start_pos, valves_space_size) = calculate(scan);
    return dp[29][start_pos][valves_space_size - 1];
}

fn second(scan: Scan) -> i32 {
    let (dp, start_pos, valves_space_size) = calculate(scan);

    return (0..valves_space_size / 2)
        .map(|path| {
            let other = valves_space_size - 1 - path;
            return dp[25][start_pos][path] + dp[25][start_pos][other];
        })
        .max()
        .unwrap();
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(parsed.clone()));
    println!("second = {}", second(parsed.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(1651, first(input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(1707, second(input));
    }
}
