use std::collections::HashSet;

use aoc_2022_rust::aoc::input::lines;

type Cube = (i32, i32, i32);

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Lava,
    Water,
}

fn parse(input: &Vec<String>) -> Vec<Cube> {
    let parse_cube = |line: &String| -> Cube {
        let parts: Vec<i32> = line.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        return (parts[0], parts[1], parts[2]);
    };
    return input.iter().map(parse_cube).collect();
}

fn all_offsets() -> Vec<(i32, i32, i32)> {
    return vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
}

fn first(cubes: &Vec<Cube>) -> usize {
    let mut map = HashSet::<Cube>::new();
    let offsets = all_offsets();

    let mut connected = 0;

    for (x, y, z) in cubes {
        for (dx, dy, dz) in &offsets {
            if map.contains(&(x + dx, y + dy, z + dz)) {
                connected += 1;
            }
        }
        map.insert((*x, *y, *z));
    }

    return 6 * cubes.len() - 2 * connected;
}

fn second(cubes: &Vec<Cube>) -> usize {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;

    for (x, y, z) in cubes {
        x_max = x_max.max(*x);
        y_max = y_max.max(*y);
        z_max = z_max.max(*z);
    }

    let offsets = all_offsets();

    let out_of_bounds = |x: i32, y: i32, z: i32| -> bool {
        return x < 0 || x > x_max + 1 || y < 0 || y > y_max + 1 || z < 0 || z > z_max + 1;
    };

    let mut map = vec![
        vec![vec![Cell::Empty; (z_max + 2) as usize]; (y_max + 2) as usize];
        (x_max + 2) as usize
    ];

    for x in [0, x_max + 1] {
        for y in [0, y_max + 1] {
            for z in [0, z_max + 1] {
                map[x as usize][y as usize][z as usize] = Cell::Water;
            }
        }
    }

    for (x, y, z) in cubes {
        map[*x as usize][*y as usize][*z as usize] = Cell::Lava;
    }

    loop {
        let mut flooded = false;
        for x in 0..x_max + 2 {
            for y in 0..y_max + 2 {
                for z in 0..z_max + 2 {
                    if map[x as usize][y as usize][z as usize] == Cell::Empty {
                        for (dx, dy, dz) in &offsets {
                            let px = x + dx;
                            let py = y + dy;
                            let pz = z + dz;

                            if out_of_bounds(px, py, pz) {
                                continue;
                            }

                            if map[px as usize][py as usize][pz as usize] == Cell::Water {
                                map[x as usize][y as usize][z as usize] = Cell::Water;
                                flooded = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if !flooded {
            break;
        }
    }

    let mut faces = 0;

    for (cx, cy, cz) in cubes {
        for (dx, dy, dz) in &offsets {
            let x = cx + dx;
            let y = cy + dy;
            let z = cz + dz;
            if out_of_bounds(x, y, z) {
                faces += 1;
            } else if map[x as usize][y as usize][z as usize] == Cell::Water {
                faces += 1;
            }
        }
    }
    return faces;
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(64, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(58, second(&input));
    }
}
