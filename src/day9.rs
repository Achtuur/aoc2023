use std::path::PathBuf;

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day9 {
    input: String,
}

impl Day for Day9 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day9.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let n = self.input.lines().map(|line| {
            let num = line.split(' ').filter_map(|n| n.parse::<i32>().ok()).collect_vec();
            let mut nums = vec![num];

            // construct tree
            while !all_elements_equal(nums.last().unwrap()) {
                let last = nums.last().unwrap();
                let mut next = Vec::new();
                for i in 1..last.len() {
                    next.push(last[i] - last[i-1])
                }
                nums.push(next);
            }

            for i in (0..nums.len() - 1).rev() {
                let add = *nums[i+1].last().unwrap();
                let cur = *nums[i].last().unwrap();
                nums[i].push(cur + add);
            }
            *nums[0].last().unwrap()
        })
        .sum::<i32>();

        Ok(n.to_string())
    }

    fn B(&mut self) -> Result<String> {
        let n = self.input.lines().map(|line| {
            let num = line.split(' ').filter_map(|n| n.parse::<i32>().ok()).collect_vec();
            let mut nums = vec![num];

            // construct tree
            while !all_elements_equal(nums.last().unwrap()) {
                let last = nums.last().unwrap();
                let mut next = Vec::new();
                for i in 1..last.len() {
                    next.push(last[i] - last[i-1])
                }
                nums.push(next);
            }

            for i in (0..nums.len() - 1).rev() {
                // forward
                let add = *nums[i+1].last().unwrap();
                let cur = *nums[i].last().unwrap();
                nums[i].push(cur + add);

                // backward
                let sub = *nums[i+1].first().unwrap();
                let cur = *nums[i].first().unwrap();
                nums[i].insert(0, cur - sub);
            }
            *nums[0].first().unwrap()
        })
        .sum::<i32>();

        Ok(n.to_string())
    }
}

fn all_elements_equal<T>(v: &[T]) -> bool 
where T: PartialEq
{
    v.iter().all(|e| *e == v[0])
}