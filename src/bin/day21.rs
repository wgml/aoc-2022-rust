use eval::Expr;
use std::collections::{HashMap, HashSet};

use aoc_2022_rust::aoc::input::lines;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Substract,
    Multiply,
    Divide,
}

impl Operation {
    fn print(&self, a: &String, b: &String) -> String {
        match self {
            Self::Add => format!("({} + {})", a, b),
            Self::Substract => format!("({} - {})", a, b),
            Self::Multiply => format!("({} * {})", a, b),
            Self::Divide => format!("({} / {})", a, b),
        }
    }
}

#[derive(Debug, Clone)]
enum Equation {
    Number(i64),
    Operation(String, Operation, String),
}

fn parse(lines: &Vec<String>) -> HashMap<String, Equation> {
    let parse_op = |op: &str| -> Operation {
        match op {
            "+" => Operation::Add,
            "-" => Operation::Substract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => panic!("unhandled op: {}", op),
        }
    };

    let parse_equation = |line: &String| -> (String, Equation) {
        let key = &line[..4];
        let eq = &line[6..];
        let parts = eq.split(' ').collect::<Vec<_>>();
        let equation = match parts.len() {
            1 => Equation::Number(parts[0].parse().unwrap()),
            3 => Equation::Operation(
                parts[0].to_string(),
                parse_op(parts[1]),
                parts[2].to_string(),
            ),
            _ => panic!("unhandled eq: {}", &eq),
        };

        return (key.to_string(), equation);
    };

    return lines.iter().map(parse_equation).collect();
}

fn solve_for_with_eval(equation: Expr, humn: i64) -> i64 {
    let expr = equation.value("humn", humn);
    return match expr.exec().unwrap() {
        eval::Value::Number(v) => v.as_f64().unwrap() as i64,
        _ => panic!(),
    };
}

fn print_for(root: &String, equations: &HashMap<String, Equation>) -> String {
    let mut deps = HashMap::<String, HashSet<&String>>::new();
    let mut prints = HashMap::<String, String>::new();

    for (s, eq) in equations {
        match eq {
            Equation::Number(_) => {
                deps.insert(s.clone(), HashSet::new());
            }
            Equation::Operation(a, _, b) => {
                deps.insert(s.clone(), HashSet::from([a, b]));
            }
        };
    }

    while !prints.contains_key(root) {
        for (key, eq) in equations {
            if deps[key].is_empty() {
                if prints.contains_key(key) {
                    continue;
                }
                let print = if key == "humn" {
                    "humn".to_string()
                } else {
                    match eq {
                        Equation::Number(v) => v.to_string(),
                        Equation::Operation(lhs, op, rhs) => op.print(&prints[lhs], &prints[rhs]),
                    }
                };
                prints.insert(key.clone(), print);
                for (_, dep) in &mut deps {
                    dep.remove(key);
                }
            }
        }
    }

    return prints[root].clone();
}

fn first(equations: &HashMap<String, Equation>) -> i64 {
    match equations["humn"] {
        Equation::Number(v) => {
            return solve_for_with_eval(Expr::new(&print_for(&"root".to_string(), equations)), v);
        }
        _ => panic!("Bad human"),
    }
}

fn second(equations: &HashMap<String, Equation>) -> i64 {
    if let Equation::Operation(lhs, _, rhs) = &equations["root"] {
        let lhs_expr = Expr::new(print_for(lhs, equations));
        let rhs_expr = Expr::new(print_for(rhs, equations));

        let is_increasing =
            solve_for_with_eval(lhs_expr.clone(), 0) < solve_for_with_eval(rhs_expr.clone(), 1);

        let mut min = i64::MIN;
        let mut max = i64::MAX;
        while min < max {
            let candidate = (max + min) / 2;
            let a = solve_for_with_eval(lhs_expr.clone(), candidate);
            let b = solve_for_with_eval(rhs_expr.clone(), candidate);

            if a == b {
                return candidate;
            }
            if (a > b) == is_increasing {
                max = candidate;
            } else {
                min = candidate;
            }
        }
        panic!("not found");
    } else {
        panic!("bad root");
    }
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(152, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(301, second(&input));
    }
}
