use std::{path::PathBuf, collections::HashMap};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day5 {
    input: String,
}

impl Day for Day5 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/test.txt");
        // let path = PathBuf::from("./input/day5.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let mut split = self.input.split("\n\n");
        let mut seeds = split.next().unwrap().split(' ').filter_map(|s| s.parse::<usize>().map_or_else(|_| None, Some)).collect_vec();
        
        split.for_each(|section| {
            let seedmaps = section.split('\n').skip(1).fold(Vec::new(), |mut vec, line| {
                let (dest, source, amount) = line.split(' ').map(|n| n.parse::<usize>().unwrap()).next_tuple().unwrap();
                vec.push(SeedMap::new(source, dest, amount));
                vec
            });

            seeds.iter_mut().for_each(|seed| {
                for seedmap in seedmaps.iter() {
                    if let Some(s) = seedmap.map_seed(seed) {
                        *seed = s;
                        break;
                    }
                }
            })
        });
        Ok(seeds.iter().min().unwrap().to_string())
    }

    fn B(&mut self) -> Result<String> {
        let mut split = self.input.split("\n\n");

        let seed_line = split.next().unwrap().split(' ').skip(1);
        let seeds = seed_line.tuples().fold(Vec::new(), |mut vec, (start, len)| {
            let (start, len) = (start.parse::<usize>().unwrap(), len.parse::<usize>().unwrap());
            vec.push(SeedRange::new(start, len));
            vec
        });

        split.for_each(|section| {
            let seedmaps = section.split('\n').skip(1).fold(Vec::new(), |mut vec, line| {
                let (dest, source, amount) = line.split(' ').map(|n| n.parse::<usize>().unwrap()).next_tuple().unwrap();
                vec.push(SeedMap::new(source, dest, amount));
                vec
            });

            // Change this to look for range instead of a single seed
            seeds.iter_mut().for_each(|seed| {
                for seedmap in seedmaps.iter() {
                    if let Some(s) = seedmap.map_seed(seed) {
                        *seed = s;
                        break;
                    }
                }
            })
        });
        Ok(seeds.iter().min().unwrap().to_string())
    }
}

impl Day5 {
    fn search_for_seed(dest: usize, maps: &[&Vec<SeedMap>], cur_map: usize, seeds: &[SeedRange]) -> Option<usize> {
        if cur_map == maps.len() {
            if seeds.iter().any(|s| s.contains(&dest)) {
                return Some(dest);
            }
            return None;
        }

        for map in maps[cur_map].iter() {
            let seed = map.reverse_map(&dest);
            if let Some(s) = seed {
                if let Some(result) = Self::search_for_seed(s, maps, cur_map + 1, seeds) {
                    return Some(result);
                }
            };
        }
        None
    }
}


struct SeedRange {
    start: usize,
    len: usize
}

impl SeedRange {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            len
        }
    }

    pub fn contains(&self, seed: &usize) -> bool {
        *seed >= self.start && *seed < self.start + self.len
    }
}

#[derive(Debug)]
struct SeedMap {
    source: usize,
    dest: usize,
    amount: usize,
}

impl SeedMap {
    pub fn new(source: usize, dest: usize, amount: usize) -> Self {
        Self {
            source,
            dest,
            amount,
        }
    }

    pub fn map_seed(&self, seed: &usize) -> Option<usize> {
        if *seed < self.source || *seed > self.source + self.amount {
            return None;
        }

        Some(seed - self.source + self.dest)
    }

    // reverse map, returns seed that would result in this dest
    pub fn reverse_map(&self, dest: &usize) -> Option<usize> {
        if *dest < self.dest || *dest > self.dest + self.amount {
            return None;
        }
        Some(dest - self.dest + self.source)
    }
}