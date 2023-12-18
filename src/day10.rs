use std::{path::PathBuf, collections::VecDeque};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result, bail};

use crate::{day::Day, util::map2d::Map2D};

#[derive(Debug, Default)]
pub struct Day10 {
    input: String,
}

impl Day for Day10 {
    fn read_input(&mut self) -> anyhow::Result<()> {
        let path = PathBuf::from("./input/test.txt");
        self.input = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn A(&mut self) -> Result<String> {
        let pipes = self.input.lines()
        .flat_map(|line| line.trim().chars())
        .map(Pipe::from_char)
        .collect_vec();

        let width = self.input.lines().next().unwrap().trim().len();
        let map = Map2D::new(pipes, width);


        let mut queue = VecDeque::<usize>::new();
        let mut visited = Map2D::new(vec![-1; map.data.len()], width);
        let start = map.data.iter().position(|x| *x == Pipe::Start).unwrap();
        visited[start] = 0;
        queue.push_back(start);

        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            let adj = map[i].dxy();
            for (dx, dy) in adj {
                if let Some(j) = map.i_from_dxy(i, dx, dy) {
                    if map.within_bounds(j) && 
                    (visited[j] < 0 || visited[i] + 1 < visited[j])
                    && map[i].can_connect_to(i, &map[j], j, map.width()) {
                        visited[j] = visited[i] + 1;
                        queue.push_back(j);
                    }
                }
            }
        };

        // map.display(1, |i| visited[i] >= 0);

        let furthest = visited.data.iter().max().unwrap();
        Ok(furthest.to_string())
    }

    fn B(&mut self) -> Result<String> {
        let pipes = self.input.lines()
        .flat_map(|line| line.trim().chars())
        .map(Pipe::from_char)
        .collect_vec();

        let width = self.input.lines().next().unwrap().trim().len();
        let map = Map2D::new(pipes, width);


        let mut queue = VecDeque::<usize>::new();
        let mut visited = Map2D::new(vec![-1; map.data.len()], width);
        let start = map.data.iter().position(|x| *x == Pipe::Start).unwrap();
        visited[start] = 0;
        queue.push_back(start);

        while !queue.is_empty() {
            let i = queue.pop_front().unwrap();
            let adj = map[i].dxy();
            for (dx, dy) in adj {
                if let Some(j) = map.i_from_dxy(i, dx, dy) {
                    if map.within_bounds(j) && 
                    (visited[j] < 0 || visited[i] + 1 < visited[j])
                    && map[i].can_connect_to(i, &map[j], j, map.width()) {
                        visited[j] = visited[i] + 1;
                        queue.push_back(j);
                    }
                }
            }
        };

        map.display(1, |i| (visited[i] >= 0) as usize);

        let mut exp_pipes = vec!['.'; map.points() * 9];
        for y in 0..map.height() {
            for x in 0..map.width() {
                let i = map.i(x, y);
                if visited[i] < 0 {
                    continue;
                }
                let exp = map[i].enlarged();

                (0..3).for_each(|n| {
                    for m in 0..3 {
                        let new_y = 3 * y + n;
                        let new_x = 3*x + m;
                        let j = new_y * width * 3 + new_x;
                        exp_pipes[j] = exp[n][m];
                    }
                });
            }
        }

        let mut exp_pipes = Map2D::new(exp_pipes, width * 3);

        exp_pipes.data = exp_pipes.data.iter().enumerate().map(|(i, x)| {
            match x {
                '#' => {
                    if exp_pipes.get_adjacent(i).iter().map(|&j| exp_pipes[j]).filter(|&n_c| n_c == '#').count() == 1 {
                        '.'
                    } else {
                        '#'
                    }
                },
                _ => *x
            }
        }).collect_vec();

        let mut inside = 0;
        let mut inside_map = Map2D::new(vec![false; exp_pipes.points()], exp_pipes.width());
        let width = 3*width;
        exp_pipes.data.iter().enumerate().for_each(|(i, p)| {

            if *p == '#' {
                return;
            }

            println!("i: {0:?}", i);


            if exp_pipes.xy(i) == (5, 5) {
                println!(" hello");
            }

            let (mut n, mut s, mut w, mut e) = (i, i, i, i);
            let mut changed = true;
            while changed {
                changed = false;
                if exp_pipes[n] != '#' && exp_pipes.y(n) > 0 && exp_pipes.same_col(i, n - width) {
                    changed = true;
                    n -= width;
                }

                if exp_pipes[s] != '#' && exp_pipes.y(s) < exp_pipes.height() - 1 && exp_pipes.same_col(i, s + width) {
                    changed = true;
                    s += width;
                }

                if exp_pipes[e] != '#' && exp_pipes.x(e) < width && exp_pipes.same_row(i, e + 1) {
                    changed = true;
                    e += 1;
                }

                if exp_pipes[w] != '#' && exp_pipes.x(w) > 0 && exp_pipes.same_row(i, w - 1) {
                    changed = true;
                    w -= 1;
                }
            }

            if exp_pipes[n] == '#' && exp_pipes[s] == '#' && exp_pipes[e] == '#' && exp_pipes[w] == '#' {
                inside += 1;
                inside_map[i] = true;
            }
        });

        exp_pipes.display(1, |i| {
            if exp_pipes[i] == '#' {
                1
            }
            else if inside_map[i] {
                2
            } else {
                0
            }
        });


        Ok(inside.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    Ne,
    Nw,
    Se,
    Sw,
    Start,
    Ground,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::Ne,
            'J' => Self::Nw,
            '7' => Self::Sw,
            'F' => Self::Se,
            'S' => Self::Start,
            _ => Self::Ground,
        }
    }

    fn enlarged(&self) -> Vec<Vec<char>> {
        match self {
            Pipe::Vertical => vec![
                vec!['.', '#', '.'],
                vec!['.', '#', '.'],
                vec!['.', '#', '.']
            ],
            Pipe::Horizontal => vec![
                vec!['.', '.', '.'],
                vec!['#', '#', '#'],
                vec!['.', '.', '.']
            ],
            Pipe::Ne => vec![
                vec!['.', '#', '.'],
                vec!['.', '#', '#'],
                vec!['.', '.', '.']
            ],
            Pipe::Nw => vec![
                vec!['.', '#', '.'],
                vec!['#', '#', '.'],
                vec!['.', '.', '.']
            ],
            Pipe::Se => vec![
                vec!['.', '.', '.'],
                vec!['.', '#', '#'],
                vec!['.', '#', '.']
            ],
            Pipe::Sw => vec![
                vec!['.', '.', '.'],
                vec!['#', '#', '.'],
                vec!['.', '#', '.']
            ],
            Pipe::Start => vec![
                vec!['.', '#', '.'],
                vec!['#', '#', '#'],
                vec!['.', '#', '.']
            ],
            Pipe::Ground => vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.']
            ],
        }
    }

    fn connects_east(&self) -> bool {
        matches!(self, Self::Horizontal | Self::Ne | Self::Se | Self::Start)
    }

    fn connects_west(&self) -> bool {
        matches!(self, Self::Horizontal | Self::Nw | Self::Sw | Self::Start)
    }

    fn connects_north(&self) -> bool {
        matches!(self, Self::Vertical | Self::Nw | Self::Ne | Self::Start)
    }

    fn connects_south(&self) -> bool {
        matches!(self, Self::Vertical | Self::Sw | Self::Se | Self::Start)
    }

    // Get dx and dy relative to the pipe_idx
    fn dxy(&self) -> Vec<(i32, i32)> {
        match self {
            Self::Vertical => vec![(0, 1), (0, -1)],
            Self::Horizontal => vec![(1, 0), (-1, 0)],
            Self::Ne => vec![(0, -1), (1, 0)],
            Self::Nw => vec![(0, -1), (-1, 0)],
            Self::Se => vec![(0, 1), (1, 0)],
            Self::Sw => vec![(0, 1), (-1, 0)],
            Self::Start => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
            Self::Ground => Vec::new(),
        }
    }

    fn can_connect_to(&self, idx: usize, other: &Self, other_idx: usize, width: usize) -> bool {
        let di = (other_idx as i32) - (idx as i32);
        let w = width as i32;

        if matches!(other, Self::Ground) {
            return false;
        }

        match self {
            Self::Ground => false,
            Self::Start => match di {
                1 => other.connects_west(),
                -1 => other.connects_east(),
                x if x == w => other.connects_north(),
                x if x == -w => other.connects_south(),
                _ => false,
            },

            Self::Horizontal => match di {
                1 => other.connects_west(),
                -1 => other.connects_east(),
                _ => false,
            },

            Self::Vertical => match di {
                x if x == w => other.connects_north(),
                x if x == -w => other.connects_south(),
                _ => false,
            }

            Self::Ne => match di {
                x if x == -w => other.connects_south(),
                1 => other.connects_west(),
                _ => false,
            }

            Self::Nw => match di {
                x if x == -w => other.connects_south(),
                -1 => other.connects_east(),
                _ => false,
            }

            Self::Se => match di {
                x if x == w => other.connects_north(),
                1 => other.connects_west(),
                _ => false,
            }

            Self::Sw => match di {
                x if x == w => other.connects_north(),
                -1 => other.connects_east(),
                _ => false,
            }
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical => write!(f, "│"),
            Pipe::Horizontal => write!(f, "─"),
            Pipe::Ne => write!(f, "└"),
            Pipe::Nw => write!(f, "┘"),
            Pipe::Sw => write!(f, "┐"),
            Pipe::Se => write!(f, "┌"),
            Pipe::Start => write!(f, "S"),
            Pipe::Ground => write!(f, "·"),
        }
    }
}
