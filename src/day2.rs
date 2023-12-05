use std::{path::PathBuf, collections::HashMap};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

const MAX_CUBES: [usize; 3] = [
    12, //r
    13, //g
    14, //b
];


#[derive(Debug)]
pub struct Day2 {
    input: String,
    thrown_reg: Regex,
}


impl Default for Day2 {
    fn default() -> Self {
        Self { 
            input: String::from(""), 
            thrown_reg: Regex::new(r#"(?<thrown>\d+)\s*(?<color>red|blue|green)"#).unwrap() 
        }
    }
}

impl Day for Day2 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day2.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let game_reg = Regex::new(r#"Game\s*(\d+):"#)?;
        Ok(self.input
        .lines()
        .map(|line| {
            let cap = game_reg.captures(line).unwrap();
            let (game_str, game_num) = (&cap[0], cap[1].parse::<u32>().unwrap());
            match self.game_is_valid(&line[game_str.len()..]) {
                true => game_num,
                false => 0,
            }
        })
        .sum::<u32>()
        .to_string())
    }

    fn B(&mut self) -> Result<String> {
        let game_reg = Regex::new(r#"Game\s*\d+:"#)?;
        Ok(self.input
        .lines()
        .map(|line| {
            let game_str = &game_reg.captures(line).unwrap()[0];
            self.get_game_power(&line[game_str.len()..]) as u32
        })
        .sum::<u32>()
        .to_string())
    }

}

impl Day2 {
    fn game_is_valid(&self, game_str: &str) -> bool {
        let game_str = &game_str.replace(' ', "");
        game_str.split(';')
        .all(|throws| {
            let mut this_game_thrown = [0; 3];
            self.thrown_reg.captures_iter(throws).all(|cap| {
                let (thrown, color) = (cap.name("thrown").unwrap().as_str().parse::<usize>().unwrap(), cap.name("color").unwrap().as_str());
                match color {
                    "red" => this_game_thrown[0] += thrown,
                    "green" => this_game_thrown[1] += thrown,
                    "blue" => this_game_thrown[2] += thrown,
                    _ => (),
                }

                if MAX_CUBES.iter().enumerate().any(|(i, max_amount)| this_game_thrown[i] > *max_amount) {
                    return false;
                }

                true
            })
        })
    }

    fn get_game_power(&self, game_str: &str) -> usize {
        let game_str = &game_str.replace(' ', "");
        let min_cubes = game_str.split(';')
        .fold([0; 3], |mut acc, throws| {
            self.thrown_reg.captures_iter(throws).for_each(|cap| {
                let (thrown, color) = (cap.name("thrown").unwrap().as_str().parse::<usize>().unwrap(), cap.name("color").unwrap().as_str());
                match color {
                    "red" => acc[0] = acc[0].max(thrown),
                    "green" => acc[1] = acc[1].max(thrown),
                    "blue" => acc[2] = acc[2].max(thrown),
                    _ => (),
                }
            });
            acc
        });

        min_cubes[0] * min_cubes[1] * min_cubes[2]
    }
}