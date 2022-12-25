use aoc_2022_rust::aoc::input::lines;

fn from_snafu(number: &String) -> u64 {
    let l = number.len();

    let mut dec = 0_i64;

    for (i, c) in number.char_indices() {
        let v: i64 = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        };
        dec += 5_i64.pow(l as u32 - 1 - i as u32) * v;
    }

    return dec as u64;
}

fn to_snafu(mut number: u64) -> String {
    let mut snafu = String::new();

    while number != 0 {
        let mut rem = number.rem_euclid(5) as i32;
        number /= 5;
        if rem == 3 {
            number += 1;
            rem = -2;
        } else if rem == 4 {
            number += 1;
            rem = -1;
        }

        snafu.push(match rem {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        });
    }
    return snafu.chars().rev().collect();
}

fn snafu_sum(numbers: &Vec<String>) -> u64 {
    return numbers.iter().map(from_snafu).sum();
}

fn first(numbers: &Vec<String>) -> String {
    let sum = snafu_sum(numbers);
    println!("sum={}", sum);
    return to_snafu(sum);
}

fn second(_: &Vec<String>) -> String {
    return "Merry Christmas".to_string();
}

fn main() {
    let parsed = lines();
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_first() {
        let input = INPUT.lines().map(|l| l.to_string()).collect();
        assert_eq!("2=-1=0", first(&input));
    }
}
