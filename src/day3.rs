use std::path::PathBuf;

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day3 {
    input: String,
    width: usize,
    height: usize,
}

impl Day for Day3 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day3.txt");
        self.input = std::fs::read_to_string(path)?;
        self.width = self.input.lines().next().unwrap().trim().len();
        self.height = self.input.lines().count();
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let mut chars = self.input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
        let mut i = 0;
        let mut sum = 0;
        while i < chars.len() {
            if !matches!(chars[i], '0'..='9' | '.') {
                sum += self.get_adjacent_numbers(&mut chars, i).iter().sum::<usize>();
            }
            i += 1;
        }
        Ok(sum.to_string())
    }

    fn B(&mut self) -> Result<String> {
        let mut chars = self.input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
        let mut i = 0;
        let mut sum = 0;
        while i < chars.len() {
            if chars[i] == '*' {
                let adj = self.get_adjacent_numbers(&mut chars, i);
                if adj.len() == 2 {
                    sum += adj[0] * adj[1]
                }
            }
            i += 1;
        }
        Ok(sum.to_string())
    }
}

impl Day3 {

    fn row(&self, i: i32) -> usize {
        (i / self.width as i32) as usize
    }

    fn within_bounds(&self, i: i32) -> bool {
        (i as usize) < self.width * self.height && i >= 0
    }

    fn get_adjacent_numbers(&self, chars: &mut [char], i: usize) -> Vec<usize> {
        get_adjacent_indices(i, self.width, self.height).iter().map(|&idx| {
            match chars[idx] {
                '0'..='9' => {
                    // If char found, go left and right until either line boundary or non-digit char is found
                    // final number will be range from j to k
                    let (mut j, mut k) = (idx as i32, idx as i32);
                    let mut changed = true;
                    while changed {
                        changed = false;
                        let next_j = j - 1;
                        if self.within_bounds(next_j) && self.row(next_j) == self.row(j) && chars[next_j as usize].is_ascii_digit() {
                            j = next_j;
                            changed = true;
                        }

                        let next_k = k + 1;
                        if self.within_bounds(next_k) && self.row(next_k) == self.row(k) && chars[next_k as usize].is_ascii_digit() {
                            k = next_k;
                            changed = true;
                        }
                    }
                    let min = j as usize;
                    let max = k as usize;

                    let num = chars[min..=max].iter().collect::<String>().parse::<usize>().unwrap();

                    (min..=max).for_each(|q| {
                        chars[q] = '.';
                    });

                    Some(num)
                }
                _ => None,
            }
        }).filter_map(|x| x.as_ref().copied()).collect_vec()
    }
}

pub fn get_adjacent_indices(cur_idx: usize, width: usize, height: usize) -> Vec<usize> {
    let mut indices = Vec::new();
    for n in 0..8 {
        indices.push(adjacent_index(cur_idx, n, width, height));
    }
    indices.iter().filter_map(|x| {
        x.as_ref().copied()
    }).collect_vec()
}

fn adjacent_index(i: usize, n: usize, width: usize, height: usize) -> Option<usize> {
    let (x, y) = (i % width, i / width);
    let o: Option<i32> = match n {
        0 if x < width - 1                          => Some(1), // right
        1 if x > 0                                  => Some(-1), // left
        2 if y < height - 1                         => Some(width as i32), // down
        3 if y > 0                                  => Some(-(width as i32)), // up
        4 if x < width - 1 && y < height - 1        => Some(1 + width as i32), // right down
        5 if x < width - 1 && y > 0                 => Some(1 - width as i32), // right up
        6 if x > 0 && y < height - 1                => Some(width as i32 - 1), // left down
        7 if x > 0 && y > 0                         => Some(-(width as i32) - 1), // left up
        _ => None,
    };


    if let Some(offset) = o {
        return Some((i as i32 + offset) as usize);
    }
    None
}


