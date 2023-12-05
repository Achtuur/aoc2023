use std::path::PathBuf;

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day4 {
    input: String,
}

impl Day for Day4 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day4.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        Ok(self.input.lines()
        .map(|line| {
            let mut line_split = line.split('|');
            let winning_numbers: Vec<usize> = line_split.next().unwrap().split(' ').filter_map(|s| s.parse::<usize>().map_or_else(|_| None, Some)).collect();
            let game_numbers: Vec<usize> = line_split.next().unwrap().split(' ').filter_map(|s| s.parse::<usize>().map_or_else(|_| None, Some)).collect();

            let mut score = None;
            winning_numbers.iter().for_each(|win_num| {
                if game_numbers.contains(win_num) {
                    match score {
                        None => score = Some(1),
                        Some(s) => score = Some(s << 1),
                    }
                }
            });
            score.unwrap_or(0)
        })
        .sum::<usize>()
        .to_string())
    }

    fn B(&mut self) -> Result<String> {
        let mut copies = vec![1; self.input.lines().count()];        

        Ok(self.input.lines()
        .enumerate()
        .map(|(i, line)| {
            let mut line_split = line.split('|');
            let winning_numbers: Vec<usize> = line_split.next().unwrap().split(' ').filter_map(|s| s.parse::<usize>().map_or_else(|_| None, Some)).collect();
            let game_numbers: Vec<usize> = line_split.next().unwrap().split(' ').filter_map(|s| s.parse::<usize>().map_or_else(|_| None, Some)).collect();

            let mut won_cards = 0;
            winning_numbers.iter().for_each(|win_num| {
                if game_numbers.contains(win_num) {
                    won_cards += 1;
                    copies[i + won_cards] += copies[i]; // add number of current card to copies of next won card
                }
            });

            copies[i] //return copies[i] here to effectively skip having to sum it up later
        })
        .sum::<usize>()
        .to_string())
    }
}
