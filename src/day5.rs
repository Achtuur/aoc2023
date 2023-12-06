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
        // let path = PathBuf::from("./input/test.txt");
        let path = PathBuf::from("./input/day5.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let mut split = self.input.split("\n\n");
        let mut seeds = split.next().unwrap().split(' ').filter_map(|s| s.parse::<usize>().ok()).collect_vec();
        
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
        let mut seeds = seed_line.tuples().fold(Vec::new(), |mut vec, (start, len)| {
            let (start, len) = (start.parse::<usize>().unwrap(), len.parse::<usize>().unwrap());
            vec.push(SeedRange::new(start, len));
            vec
        });

        split.for_each(|section| {
            let mut seedmaps = section.split('\n').skip(1).fold(Vec::new(), |mut vec, line| {
                let (dest, source, amount) = line.split(' ').map(|n| n.parse::<usize>().unwrap()).next_tuple().unwrap();
                vec.push(SeedMap::new(source, dest, amount));
                vec
            });
            
            // Fill in gaps in seedmap
            seedmaps.sort_by(|a, b| a.source.cmp(&b.source));
            if seedmaps[0].source > 0 {
                seedmaps.insert(0, SeedMap::new(0, 0, seedmaps[0].source));
            }

            let mut i = 1;
            while i < seedmaps.len() - 1{
                let (s1, s2) = (&seedmaps[i], &seedmaps[i+1]);
                if s1.source + s1.amount < s2.source {
                    let gap_source = s1.source + s1.amount;
                    let amount = s2.source - gap_source;
                    let gap_dest = gap_source;
                    seedmaps.insert(i + 1, SeedMap::new(gap_source, gap_dest, amount));
                    i += 1;
                }
                i += 1;
            }
            
            let mut i = 0;
            while i < seeds.len() {
                let fitting_map = seedmaps.iter().find(|map| map.map_range(&seeds[i]).0.is_some());
                if let Some(map) = fitting_map {
                    let (mapped, unmapped) = map.map_range(&seeds[i]);
                    if let Some(um) = unmapped {
                        seeds.push(um);
                    }
                    if let Some(m) = mapped {
                        seeds[i] = m;
                    }
                }
                i += 1;
            }

        });

        seeds.sort_by(|a, b| a.start.cmp(&b.start));
        seeds.dedup();
        Ok(seeds[0].start.to_string())
    }
}

// ANSWER IS BETWEEN 17164440 AND 167312469

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

    // Returns (mapped_range, unmapped_range) pair
    pub fn map_range(&self, seed_range: &SeedRange) -> (Option<SeedRange>, Option<SeedRange>) {
        let (s1, e1) = (self.source, self.source + self.amount);
        let (s2, e2) = (seed_range.start, seed_range.start + seed_range.len);
        // no overlap
        if s2 < s1 && e2 < s1 || s2 > e1 && e2 > e1 {
            return (None, Some(seed_range.clone()));
        }
        // complete overlap
        else if s2 >= s1 && e2 <= e1 {
            let start = self.map_seed(&s2).unwrap();
            return (Some(SeedRange::new(start, seed_range.len)), None);
        }
        // partial overlap on the left
        else if s2 < s1 && e2 >= s1 {
            let start = self.map_seed(&s1).unwrap();
            return (Some(SeedRange::new(start, e1 - s1)), Some(SeedRange::new(s2, s1 - s2)));
        }
        // partial overlap on the right
        else if s2 < e1 && e2 > e1 {
            let start = self.map_seed(&s2).unwrap();
            return (Some(SeedRange::new(start, e1 - s2)), Some(SeedRange::new(e1, e2 - e1)))
        }
        (None, None)
    }
}