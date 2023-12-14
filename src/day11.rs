use std::{path::PathBuf, collections::VecDeque};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::{day::Day, util::map2d::Map2D};

#[derive(Debug, Default)]
pub struct Day11 {
    input: String,
    exp_fac: usize,
}

impl Day for Day11 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/test.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        self.exp_fac = 2;
        let map = self.input.lines().flat_map(|l| l.trim().chars().map(|c| c.to_string())).collect_vec();
        let width = self.input.lines().next().unwrap().trim().len();
        let mut map = Map2D::new(map, width);

        // prepare map
        let (mut add_rows, mut add_cols) = (Vec::new(), Vec::new());

        for i in 0..map.width().max(map.height()) {
            if (0..map.width()).all(|x| map[(x, i)] == ".") {
                add_rows.push(i); // add length to account for added rows
            }

            if (0..map.height()).all(|y| map[(i, y)] == ".") {
                add_cols.push(i); 
            }
        }

        for i in add_rows {
            map.replace_row(i, &"@".to_owned());
        }

        for i in add_cols {
            map.replace_column(i, &"@".to_owned());
        }

        let starting_pos = map.data.iter().enumerate().filter_map(|(i, x)| if x != "." {Some(i)} else {None}).collect_vec();

        let res = (0..starting_pos.len()).map(|i| {
            (i+1..starting_pos.len()).map(|j| {
                self.shortest_path2(&map, starting_pos[i], starting_pos[j])
            }).sum::<usize>()
        }).sum::<usize>();


        Ok(res.to_string())
    }

    fn B(&mut self) -> Result<String> {
        self.exp_fac = 10;
        let map = self.input.lines().flat_map(|l| l.trim().chars().map(|c| c.to_string())).collect_vec();
        let width = self.input.lines().next().unwrap().trim().len();
        let mut map = Map2D::new(map, width);

        // prepare map
        let (mut add_rows, mut add_cols) = (Vec::new(), Vec::new());

        for i in 0..map.width().max(map.height()) {
            if (0..map.width()).all(|x| map[(x, i)] == ".") {
                add_rows.push(i); // add length to account for added rows
            }

            if (0..map.height()).all(|y| map[(i, y)] == ".") {
                add_cols.push(i); 
            }
        }

        for i in add_rows {
            map.replace_row(i, &"@".to_owned());
        }

        for i in add_cols {
            map.replace_column(i, &"@".to_owned());
        }

        println!("{}", map);

        let starting_pos = map.data.iter().enumerate().filter_map(|(i, x)| if x == "#" {Some(i)} else {None}).collect_vec();

        let res = (0..starting_pos.len()).map(|i| {
            (i+1..starting_pos.len()).map(|j| {
                self.shortest_path2(&map, starting_pos[i], starting_pos[j])
            }).sum::<usize>()
        }).sum::<usize>();


        Ok(res.to_string())
    }
}


impl Day11 {   
    fn shortest_path2(&self, map: &Map2D<String>, start: usize, end: usize) -> usize {
        let (sx, sy) = map.xy(start);
        let (ex, ey) = map.xy(end);

        let mut distance = 0;
        
        let y = sy;
        for x in sx.min(ex)..=sx.max(ex) {
            distance += match map[(x, y)].as_str() {
                "@" => self.exp_fac,
                _ => 1,
            }
        }
        
        let x = ex;
        for y in sy.min(ey)..=sy.max(ey) {
            distance += match map[(x, y)].as_str() {
                "@" => self.exp_fac,
                _ => 1,
            }
        }
        
        map.display(2, |i| {
            if (sx.min(ex)..=sx.max(ex)).contains(&map.x(i)) && map.y(i) == sy {
                1
            }
            else if (sy.min(ey)..=sy.max(ey)).contains(&map.y(i)) && map.x(i) == ex {
                2
            } else {
                0
            }
        });
        println!("{}", distance);

        distance
    }
}

fn shortest_path(map: &Map2D<String>, start: usize, end: usize) -> usize {
    let mut queue = VecDeque::<usize>::new();
    let mut visited = Map2D::new(vec![-1_i32; map.data.len()], map.width());
    visited[start] = 0;
    queue.push_back(start);

    while !queue.is_empty() {
        let i = queue.pop_front().unwrap();
        for j in map.get_adjacent(i) {
            if map.within_bounds(j) && 
            (visited[j] < 0 || visited[i] + 1 < visited[j]) {
                if j == end {
                    return visited[i] as usize + 1;
                }

                visited[j] = visited[i] + 1;
                queue.push_back(j);
            }
        }
    };
    usize::MAX
}
