use std::{cmp::min, collections::HashMap, slice::Iter};

use aoc_2022_rust::aoc::input::lines;

struct File {
    size: usize,
}

struct Directory {
    dirs: HashMap<String, Directory>,
    // files: HashMap<String, File>, // this is not needed for the solution
    size: usize,
}

fn ls_dir(iter: &mut Iter<String>) -> Directory {
    let mut dirs = HashMap::new();
    let mut files = HashMap::new();

    while let Some(line) = iter.next() {
        match &line[..4] {
            "$ cd" => {
                let dir_name = &line[5..];
                if dir_name == ".." {
                    break;
                }
                dirs.insert(dir_name.to_string(), ls_dir(iter));
            }
            "$ ls" => continue,
            "dir " => continue,
            _ => {
                let (size, name) = line.split_once(' ').unwrap();
                files.insert(
                    name.to_string(),
                    File {
                        size: size.parse().unwrap(),
                    },
                );
            }
        }
    }

    let size = dirs.iter().map(|(_, d)| d.size).sum::<usize>()
        + files.iter().map(|(_, f)| f.size).sum::<usize>();
    return Directory { dirs, size };
}

fn parse(input: &Vec<String>) -> Directory {
    return ls_dir(&mut input.iter());
}

fn sum_dirs_below(root: &Directory, limit: usize) -> usize {
    let mut sum = 0;
    if root.size <= limit {
        sum += root.size;
    }
    for (_, d) in &root.dirs {
        sum += sum_dirs_below(&d, limit);
    }

    return sum;
}

fn first(root: &Directory) -> usize {
    return sum_dirs_below(root, 100000);
}

fn find_best_candidate(root: &Directory, limit_to_free: usize) -> Option<usize> {
    if root.size < limit_to_free {
        return None;
    }

    let mut candidate = root.size;

    for (_, dir) in &root.dirs {
        match find_best_candidate(dir, limit_to_free) {
            Some(v) => candidate = min(candidate, v),
            None => continue,
        }
    }

    return Some(candidate);
}

fn second(root: &Directory) -> usize {
    let disk_space: usize = 70000000;
    let required_unused: usize = 30000000;
    let currently_unused = disk_space - root.size;

    return find_best_candidate(root, required_unused - currently_unused).unwrap();
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(&parsed));
    println!("second = {}", second(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(95437, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(24933642, second(&input));
    }
}
