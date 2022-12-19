use aoc_2022_rust::aoc::input::lines;

type Cost = [u16; 4];

struct Blueprint {
    costs: [Cost; 4],
}

type Input = Vec<Blueprint>;

fn parse(input: &Vec<String>) -> Input {
    let parse_line = |line: &String| -> Blueprint {
        let parts: Vec<u16> = line
            .split(' ')
            .map(|p| p.parse::<u16>())
            .filter(|n| n.is_ok())
            .flat_map(|n| n)
            .collect();

        return Blueprint {
            costs: [
                [parts[0], 0, 0, 0],
                [parts[1], 0, 0, 0],
                [parts[2], parts[3], 0, 0],
                [parts[4], 0, parts[5], 0],
            ],
        };
    };

    return input.iter().map(parse_line).collect();
}

struct State {
    resources: [u16; 4],
    robots: [u16; 4],
    time: u16,
}

fn optimize_dfs(
    blueprint: &Blueprint,
    state: State,
    max_per_kind: &[u16; 4],
    max_time: u16,
    current_best: u16,
) -> u16 {
    let wait_time = |cost: &Cost| -> u16 {
        (0..3)
            .filter_map(|kind| {
                if cost[kind] == 0 {
                    None
                } else if cost[kind] <= state.resources[kind] {
                    Some(0)
                } else if state.robots[kind] != 0 {
                    Some(
                        (cost[kind] - state.resources[kind] + state.robots[kind] - 1)
                            / state.robots[kind],
                    )
                } else {
                    Some(max_time + 1)
                }
            })
            .max()
            .unwrap()
    };

    let maybe_optimal =
        |remaining_time: u16, resources: &[u16; 4], robots: &[u16; 4], best_result: u16| -> bool {
            return (remaining_time - 1) * remaining_time / 2
                + resources[3]
                + remaining_time * robots[3]
                >= best_result;
        };

    let mut best_result = std::cmp::max(
        current_best,
        state.resources[3] + state.robots[3] * (max_time - state.time),
    );

    for kind in 0..=3 {
        if state.robots[kind] >= max_per_kind[kind] {
            continue;
        }

        let delay = wait_time(&blueprint.costs[kind]);

        let wait_until = state.time + delay + 1;
        if wait_until >= max_time {
            continue;
        }

        let mut new_resources = [0; 4];
        let mut new_robots = [0; 4];

        for k in 0..=3 {
            new_resources[k] =
                state.resources[k] + state.robots[k] * (delay + 1) - blueprint.costs[kind][k];
            new_robots[k] = state.robots[k] + if kind == k { 1 } else { 0 };
        }

        let remaining = max_time - wait_until;
        if maybe_optimal(remaining, &new_resources, &new_robots, best_result) {
            best_result = best_result.max(optimize_dfs(
                blueprint,
                State {
                    resources: new_resources,
                    robots: new_robots,
                    time: wait_until,
                },
                max_per_kind,
                max_time,
                best_result,
            ));
        }
    }

    return best_result;
}

fn optimize_for_geodes(blueprint: &Blueprint, max_time: u16) -> u16 {
    let mut max_per_kind = [0; 4];

    for i in 0..3 {
        max_per_kind[i] = blueprint.costs.iter().map(|c| c[i]).max().unwrap();
    }
    max_per_kind[3] = u16::MAX;

    return optimize_dfs(
        blueprint,
        State {
            resources: [0; 4],
            robots: [1, 0, 0, 0],
            time: 0,
        },
        &max_per_kind,
        max_time,
        0,
    );
}

fn first(blueprints: &Input) -> u16 {
    return blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i as u16 + 1) * optimize_for_geodes(b, 24))
        .sum();
}

fn second(blueprints: &Input) -> u16 {
    return blueprints
        .iter()
        .take(3)
        .map(|b| optimize_for_geodes(b, 32))
        .product();
}
fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(33, first(&input));
    }
}
