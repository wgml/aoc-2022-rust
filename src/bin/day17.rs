use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use aoc_2022_rust::aoc::input::as_str;

#[derive(Clone, Debug)]
struct Buffer<T> {
    elements: Vec<T>,
    next: usize,
}

type Offset = (i32, i32);

#[derive(Debug)]
struct Rock {
    rocks: Vec<Offset>,
}

type Chamber = Vec<u8>;
type Jets = Buffer<char>;

type CacheState = (usize, usize, u64);

impl Rock {
    fn all() -> Buffer<Rock> {
        let horizontal = Rock {
            rocks: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        };
        let plus = Rock {
            rocks: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        };
        let l = Rock {
            rocks: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        };
        let vertical = Rock {
            rocks: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        };
        let square = Rock {
            rocks: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        };

        return Buffer::from(vec![horizontal, plus, l, vertical, square]);
    }

    fn try_move(&self, chamber: &Chamber, position: &Offset, offset: &Offset) -> Option<Offset> {
        let (px, py) = position;
        let (ox, oy) = offset;

        for (rx, ry) in &self.rocks {
            let x = rx + px + ox;
            let y = (ry + py + oy) as usize;

            if x < 0 || x >= 7 {
                return None;
            }
            if chamber.len() > y && chamber[y] & (1 << x as u8) != 0 {
                return None;
            }
        }

        return Some((px + ox, py + oy));
    }

    fn try_push(&self, chamber: &Chamber, position: &Offset, jet: &char) -> Offset {
        let offset = match jet {
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => panic!("Unhandled {}", jet),
        };

        if let Some(new_pos) = self.try_move(chamber, position, &offset) {
            return new_pos;
        }

        return position.clone();
    }

    fn try_fall(&self, chamber: &Chamber, position: &Offset) -> Option<Offset> {
        let offset = (0, -1);

        return self.try_move(chamber, position, &offset);
    }

    fn freeze(&self, chamber: &mut Chamber, position: &Offset) {
        let (px, py) = position;
        for (rx, ry) in &self.rocks {
            let x = (px + rx) as u8;
            let y = (ry + py) as usize;

            while chamber.len() <= y {
                chamber.push(0);
            }

            chamber[y] |= 1 << x;
        }

        let to_add = chamber.iter().rev().take(3).filter(|r| **r != 0).count();
        for _ in 0..to_add {
            chamber.push(0);
        }
    }
}

impl<T> Buffer<T> {
    fn from(elements: Vec<T>) -> Buffer<T> {
        return Buffer { elements, next: 0 };
    }

    fn next(&mut self) -> &T {
        let i = self.next;
        self.next = (self.next + 1) % self.elements.len();
        return &self.elements[i];
    }
}

fn parse(input: &str) -> Jets {
    return Jets::from(input.chars().collect());
}

fn hash_chamber(chamber: &Chamber) -> u64 {
    let mut s = DefaultHasher::new();

    for r in chamber.iter().rev().take(11) {
        // 11 is enough, apparently 🤷‍♂️
        r.hash(&mut s);
    }

    return s.finish();
}

fn chamber_height(chamber: &Chamber) -> usize {
    let empty = chamber.iter().rev().take_while(|r| **r == 0).count();
    return chamber.len() - empty - 1;
}

fn simulate(mut jets: Jets, limit: i64) -> usize {
    let mut rocks = Rock::all();
    let mut chamber = vec![127, 0, 0, 0];

    let mut cache = HashMap::<CacheState, (i64, usize)>::new();
    let mut inc_height = 0 as usize;

    let mut i = 0 as i64;
    while i < limit {
        let state = (rocks.next, jets.next, hash_chamber(&chamber));
        let height = chamber_height(&chamber);
        if let Some(prev) = cache.get(&state) {
            let (prev_i, prev_h) = prev;
            let diff_i = i - prev_i;
            let diff_h = height - prev_h;

            let times = (limit - i) / diff_i;
            inc_height += times as usize * diff_h;
            i += times * diff_i;
        } else {
            cache.insert(state, (i, chamber_height(&chamber)));
        }

        let rock = rocks.next();
        let mut position: Offset = (2, chamber.len() as i32);
        loop {
            position = rock.try_push(&chamber, &position, jets.next());
            if let Some(new_position) = rock.try_fall(&mut chamber, &position) {
                position = new_position;
            } else {
                rock.freeze(&mut chamber, &position);
                break;
            }
        }

        i += 1;
    }

    return inc_height + chamber_height(&chamber);
}

fn first( jets: Jets) -> usize {
    simulate(jets, 2022)
}


fn second(jets: Jets) -> usize {
    simulate(jets, 1000000000000)
}

fn main() {
    let parsed = parse(&as_str());
    println!("first = {}", first(parsed.clone()));
    println!("second = {}", second(parsed.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_first() {
        let input = parse(&INPUT);
        assert_eq!(3068, first(input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT);
        assert_eq!(1514285714288, second(input));
    }
}
