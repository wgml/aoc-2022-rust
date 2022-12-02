use aoc_2022_rust::aoc::input::lines;

#[derive(Clone)]
enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

enum Outcome {
    Win,
    Draw,
    Loose,
}

type Round = (Choice, Choice, Outcome);

fn parse_round(line: &String) -> Round {
    let to_choice = |c: char| match c {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        _ => panic!("Unhandled choice: {}!", c),
    };
    let to_outcome = |c: char| match c {
        'X' => Outcome::Loose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Unhandled choice: {}!", c),
    };

    return (
        to_choice(line.chars().nth(0).unwrap()),
        to_choice(line.chars().nth(2).unwrap()),
        to_outcome(line.chars().nth(2).unwrap()),
    );
}

fn parse(input: Vec<String>) -> Vec<Round> {
    input.iter().map(parse_round).collect()
}

fn to_ordinal(choice: &Choice) -> u32 {
    choice.clone() as u32
}

fn from_ordinal(ord: u32) -> Choice {
    match ord % 3 {
        0 => Choice::Rock,
        1 => Choice::Paper,
        2 => Choice::Scissors,
        _ => panic! {"unreachable"},
    }
}

fn kind_score(choice: &Choice) -> u32 {
    return to_ordinal(choice) + 1;
}

fn outcome_score(opponent: &Choice, you: &Choice) -> u32 {
    let diff = (3 + to_ordinal(you) - to_ordinal(opponent)) % 3;
    return ((diff + 1) % 3) * 3;
}

fn score((opponent, you): (&Choice, &Choice)) -> u32 {
    kind_score(you) + outcome_score(opponent, you)
}

fn first<'a>(rounds: &'a Vec<Round>) -> u32 {
    let make_choice = |(opponent, you, _): &'a Round| (opponent, you);

    return rounds.iter().map(make_choice).map(score).sum();
}

fn second<'a>(rounds: &'a Vec<Round>) -> u32 {
    let make_choice = |(opponent, _, outcome): &'a Round| -> (&'a Choice, Choice) {
        let you = match outcome {
            Outcome::Draw => opponent.clone(),
            Outcome::Loose => from_ordinal((to_ordinal(opponent) + 2) % 3),
            Outcome::Win => from_ordinal(to_ordinal(opponent) + 4),
        };
        return (opponent, you);
    };

    return rounds
        .iter()
        .map(make_choice)
        .map(|round| score((&round.0, &round.1)))
        .sum();
}

fn main() {
    let parsed = parse(lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    fn test_input() -> Vec<Round> {
        return parse(INPUT.split('\n').map(|l| l.to_string()).collect());
    }

    #[test]
    fn test_first() {
        assert_eq!(15, first(&test_input()));
    }

    #[test]
    fn test_second() {
        assert_eq!(12, second(&test_input()));
    }
}
