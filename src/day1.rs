use std::path::PathBuf;

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day1 {
    input: String,
}

impl Day for Day1 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day1.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        Ok(self.input
        .lines()
        .map(|line| {
            let line_num = line
            .chars()
            .filter(|c| 
                c.is_numeric()
            )
            .collect::<String>();
            format!("{}{}", line_num.chars().nth(0).unwrap(), line_num.chars().last().unwrap())
        })
        .map(|line_num| line_num.parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string())
    }

    fn B(&mut self) -> Result<String> {
        Ok(self.input
        .lines()
        .map(|line| {
            let line_num = line_to_nums(line).unwrap();
            println!("(line, line_num): {0:?}", (line, &line_num, line_num.chars().nth(0).unwrap().to_string(), format!("{}{}", line_num.chars().nth(0).unwrap(), line_num.chars().last().unwrap())));
            format!("{}{}", line_num.chars().nth(0).unwrap(), line_num.chars().last().unwrap())
        })
        .map(|line_num| line_num.parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string())
    }
}

const NUMS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn line_to_nums(line: &str) -> Result<String> {
    // let reg = Regex::new(r#"(?=(one|two|three|four|five|six|seven|eight|nine|\d))"#).unwrap();

    let mut s = "".to_owned();
    for i in 0..line.len() {
        if line.chars().nth(i).unwrap().is_numeric() {
            s += line.chars().nth(i).unwrap().to_string().as_str();
            continue;
        }

        for num in NUMS {
            if line[i..].starts_with(num.0) {
                s += num.1;
            }
        }
    }
    Ok(s)
}