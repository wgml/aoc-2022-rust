pub mod aoc {
    pub mod input {
        use std::io::{self, BufRead};

        pub fn as_str() -> String {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("no input");
            return buffer
        }

        pub fn single_str() -> String {
            let stdin = io::stdin();
            return stdin.lock().lines().next().unwrap().unwrap()
        }
    }
}
