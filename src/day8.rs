use std::{path::PathBuf, collections::HashMap};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::day::Day;

#[derive(Debug, Default)]
pub struct Day8 {
    input: String,
    instr: Vec<char>,
    graph: HashMap<String, (String, String)>,
}

impl Day for Day8 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/day8.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let mut lines = self.input.lines();
        self.instr = lines.next().unwrap().chars().collect_vec();
        let reg = Regex::new(r#"(?<root>\w+) = \((?<left>\w+), (?<right>\w+)\)"#).unwrap();
        // read graph

        self.graph = lines.fold(HashMap::new(),|mut map, line| {
            match reg.captures(line) {
                Some(cap) => {
                    map.insert(cap["root"].to_string(), (cap["left"].to_string(), cap["right"].to_string()));
                }
                None => println!("invalid input: {0:?}", line)
            }
            map
        });

        let steps = self.get_path_len("AAA", |node| node == "ZZZ");
        Ok(steps.to_string())
    }

    fn B(&mut self) -> Result<String> {
        let mut lines = self.input.lines();
        self.instr = lines.next().unwrap().chars().collect_vec();
        let reg = Regex::new(r#"(?<root>\w+) = \((?<left>\w+), (?<right>\w+)\)"#).unwrap();
        self.graph = lines.fold(HashMap::new(),|mut map, line| {
            match reg.captures(line) {
                Some(cap) => {
                    map.insert(cap["root"].to_string(), (cap["left"].to_string(), cap["right"].to_string()));
                }
                None => println!("invalid input: {0:?}", line)
            }
            map
        });

        let starting_points = self.graph.iter().filter(|(root, _)| root.ends_with('A'))
        .map(|(root, _)| root)
        .collect_vec();

        let steps = starting_points.iter()
        .map(|start| self.get_path_len(start, |node| node.ends_with('Z')))
        .collect_vec();

        println!("steps: {0:?}", steps);

        let lcm = steps.iter().fold(steps[0], |acc, &step| lcm(acc, step));
        Ok(lcm.to_string())
    }
}

impl Day8 {
    fn get_path_len<F>(&self, starting_point: &str, cmp: F) -> usize
    where F: Fn(&str) -> bool
    {
        let mut instr_ptr = 0;
        let mut steps = 0;
        let mut cur_node = starting_point;
        loop {
            steps += 1;
            match self.instr[instr_ptr] {
                'L' => cur_node = &self.graph[cur_node].0,
                'R' => cur_node = &self.graph[cur_node].1,
                _ => println!("invalid instruction")
            }

            if cmp(cur_node) {
                return steps;
            }

            instr_ptr = (instr_ptr + 1) % self.instr.len();
        }
    }
}


// wikipedia: https://en.wikipedia.org/wiki/Binary_GCD_algorithm
fn gcd(a: usize, b: usize) -> usize {
    // `wrapping_abs` gives a number's absolute value, unless that's 2³¹. 2³¹
    // won't fit in `i32`, so it gives -2³¹ instead.
    let mut v = b;
    if a == 0 {
        return v;
    }
    let mut u = a;
    if b == 0 {
        return u;
    }

    // `|` is bitwise OR. `trailing_zeros` quickly counts a binary number's
    // trailing zeros, giving its prime factorization's exponent on two.
    let gcd_exponent_on_two = (u | v).trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in
    // the left variable. `u` divided by its prime factorization's power of two
    // turns it odd.
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right.
    u << gcd_exponent_on_two
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}