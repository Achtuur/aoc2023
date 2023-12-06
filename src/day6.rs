use std::path::PathBuf;

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day6 {
    input: String,
}

impl Day for Day6 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day6.txt");
        // let path = PathBuf::from("./input/test.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let mut split = self.input.lines();
        let times = split.next().unwrap().split(' ').skip(1).filter_map(|s| s.parse::<usize>().ok()).collect_vec();
        let distances = split.next().unwrap().split(' ').skip(1).filter_map(|s| s.parse::<usize>().ok()).collect_vec();


        // we have a function d(t) = p(t-p) where p is the time the button was pressed and t is the current time
        // For the last value, d(max_time) = max_distance -> t_max = floor(max_time)
        // for the first value, d(min_time) = max_distance -> t_min = ceil(min_time)
        // eg there are 2 p's for which d(t) is exactly max_distance
        // using some simple math:
        // p^2 - pt + max_distance = 0
        // p0 = (t - sqrt(t^2 - 4*max_distance)) / 2
        // p1 = (t + sqrt(t^2 - 4*max_distance)) / 2

        Ok(
            times.iter().zip(distances.iter()).map(|(time, distance)| {
                let (t, d) = (*time as f64, *distance as f64);
                let sqrt_d = (t*t - 4.0*d).sqrt();
                let p0 = 0.5 * (t - sqrt_d) + 0.001; // add a very small constant so this always rounds up, even when 'exactly' on the number
                let p1 = 0.5 * (t + sqrt_d) - 0.001;
                println!("(p0, p1): {0:?}", (p0, p1));
                println!("races: {0:?}", (p1.floor() - p0.ceil()));
                (p1.floor() - p0.ceil()) as usize + 1
            })
            .product::<usize>().to_string()
        )
    }

    fn B(&mut self) -> Result<String> {
        let mut split = self.input.lines();
        let time = split.next().unwrap().split(' ').skip(1).filter_map(|s| s.parse::<usize>().ok()).map(|x| x.to_string()).join("").parse::<usize>().unwrap();
        let distance = split.next().unwrap().split(' ').skip(1).filter_map(|s| s.parse::<usize>().ok()).map(|x| x.to_string()).join("").parse::<usize>().unwrap();

        let (t, d) = (time as f64, distance as f64);
        let sqrt_d = (t*t - 4.0*d).sqrt();
        let p0 = 0.5 * (t - sqrt_d) + 0.001; // add a very small constant so this always rounds up, even when 'exactly' on the number
        let p1 = 0.5 * (t + sqrt_d) - 0.001;
        println!("(p0, p1): {0:?}", (p0, p1));
        println!("races: {0:?}", (p1.floor() - p0.ceil()));
        let wins = (p1.floor() - p0.ceil()) as usize + 1;
        Ok(wins.to_string())
    }
}
