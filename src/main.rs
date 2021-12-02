use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

use aoc2021::{build_solutions, Part, Solution};

fn read_input(day: String) -> Result<Vec<String>, Error> {
    let path = Path::new("src").join(day).join("input.txt");
    BufReader::new(File::open(path)?).lines().collect()
}

fn main() {
    let solutions = build_solutions();

    let run_fn = |day: &String, sol: &Solution| {
        println!("Running {}", day);
        let input = read_input(day.to_string()).unwrap();
        for part in [Part::One, Part::Two] {
            print!("    {}...", part);
            if sol.run(part, &input) {
                println!("passed")
            } else {
                println!("failed")
            }
        }
    };

    match env::args().nth(1) {
        Some(day) => {
            if day.eq("all") {
                println!("Run all days");
                for (day, sol) in solutions.iter() {
                    run_fn(day, sol);
                }
            } else {
                let day = if let Ok(day) = day.parse::<u32>() {
                    format!("day{:02}", day)
                } else {
                    day
                };
                if let Some(sol) = solutions.get(&day) {
                    run_fn(&day, sol);
                } else {
                    eprintln!("WTF: {}", day);
                }
            }
        }
        None => {
            if let Some(day) = solutions.keys().last() {
                let sol = solutions.get(day).unwrap();
                run_fn(day, sol);
            } else {
                eprintln!("no solution")
            }
        }
    }
}
