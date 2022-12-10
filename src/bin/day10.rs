use aoc_2022_rust::aoc::input::lines;

enum Instruction {
    Noop,
    Addx(i32),
}

type Input = Vec<Instruction>;

fn parse(input: &Vec<String>) -> Input {
    let mut result = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "noop" => result.push(Instruction::Noop),
            "addx" => result.push(Instruction::Addx(parts[1].parse().unwrap())),
            _ => panic!("Unhandled {}", line),
        }
    }

    return result;
}

fn first(instructions: &Input) -> i32 {
    let mut cycle = 1;
    let mut pc = 0;
    let mut current_instr_cycles = 0;
    let mut signal_strength = 0;
    let mut register = 1;

    while pc < instructions.len() && cycle <= 220 {
        if cycle % 40 == 20 {
            signal_strength += cycle * register;
        }

        match &instructions[pc] {
            Instruction::Noop => pc += 1,
            Instruction::Addx(x) => {
                if current_instr_cycles == 0 {
                    current_instr_cycles += 1
                } else {
                    register += x;
                    pc += 1;
                    current_instr_cycles = 0;
                }
            }
        }

        cycle += 1;
    }

    return signal_strength;
}

fn second(instructions: &Input) -> String {
    let mut cycle = 0;
    let mut pc = 0;
    let mut current_instr_cycles = 0;
    let mut register: i32 = 1;

    let mut screen: Vec<Vec<bool>> = Vec::new();
    for _ in 0..6 {
        screen.push(vec![false; 40]);
    }

    while pc < instructions.len() {
        let row = cycle / 40;
        let col = cycle % 40;
        if register == col || register == col - 1 || register == col + 1 {
            screen[row as usize][col as usize] = true;
        }

        match &instructions[pc] {
            Instruction::Noop => pc += 1,
            Instruction::Addx(x) => {
                if current_instr_cycles == 0 {
                    current_instr_cycles += 1
                } else {
                    register += x;
                    pc += 1;
                    current_instr_cycles = 0;
                }
            }
        }

        cycle += 1;
    }

    return screen
        .iter()
        .map(|v| {
            v.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        })
        .fold(String::from("\n"), |a, b| a + &b + "\n");
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(13140, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(
            r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
",
            second(&input)
        );
    }
}
