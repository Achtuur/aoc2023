use std::path::Display;

use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Map2D<T> {
    pub data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Map2D<T> {
    pub fn new(data: Vec<T>, width: usize) -> Self {
        let height = data.len() / width;
        Self { data, width, height }
    }

    pub fn points(&self) -> usize{
        self.width() * self.height()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn i(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn x(&self, i: usize) -> usize {
        i % self.width
    }

    pub fn y(&self, i: usize) -> usize {
        i / self.width
    }

    pub fn xy(&self, i: usize) -> (usize, usize) {
        (self.x(i), self.y(i))
    }

    pub fn i_from_dxy(&self, i: usize, dx: i32, dy: i32) -> Option<usize> {
        let j = (i as i32 + dx + dy * self.width as i32) as usize;
        if self.within_bounds(j) && (self.same_row(i, j) || self.same_col(i, j)) {
            return Some(j);
        }
        None
    }

    pub fn same_row(&self, i: usize, j: usize) -> bool {
        self.y(i) == self.y(j)
    }

    pub fn same_col(&self, i: usize, j: usize) -> bool {
        self.x(i) == self.x(j)
    }

    pub fn within_bounds(&self, i: usize) -> bool {
        let (x, y) = self.xy(i);
        x < self.width && y < self.height
    }

    pub fn within_bounds_xy(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_adjacent(&self, i: usize) -> Vec<usize> {
        let w = self.width as i32;
        self.get_adjacent_act(i, &[1, -1, w, -w]).collect_vec()
    }

    pub fn get_adjacent_diagonal(&self, i: usize) -> Vec<usize> {
        let w = self.width as i32;
        self.get_adjacent_act(i, &[1, -1, w, -w, w + 1, w - 1, -w + 1, -w - 1]).collect_vec()
    }

    fn get_adjacent_act<'a>(&'a self, i: usize, di: &'a [i32]) -> impl Iterator<Item = usize> + '_ {
        di.iter()
        .map(move |d| i as i32 + d)
        .map(move |j| j as usize)
        .filter(move |j| self.within_bounds(*j) && (self.same_row(i, *j) || self.same_col(i, *j)))
    }


    pub fn insert_row(&mut self, y: usize, data: &T) 
    where T: Clone
    {
        for _ in 0..self.width {
            self.data.insert(self.i(0, y), data.clone());
        }
        self.height += 1;
    }

    pub fn insert_column(&mut self, x: usize, data: &T) 
    where T: Clone
    {
        for y in 0..self.height {
            self.data.insert(self.i(x + y, y), data.clone());
        }
        self.width += 1;
    }

    pub fn replace_row(&mut self, y: usize, data: &T) 
    where T: Clone
    {
        for x in 0..self.width() {
            self[(x, y)] = data.clone();
        }
    }

    pub fn replace_column(&mut self, x: usize, data: &T) 
    where T: Clone
    {
        for y in 0..self.height() {
            self[(x, y)] = data.clone();
        }
    }

    pub fn display(&self, max_width: usize, color: impl Fn(usize) -> usize) 
    where T: std::fmt::Display
    {
        let data_vec = self.data.iter().enumerate().map(|(i, x)| {
            match color(i) {
                0 => format!("{}", x).white(),
                1 => format!("{}", x).bright_cyan(),
                2 => format!("{}", x).bright_red(),
                _ => format!("{}", x).black(),
            }
        }).collect_vec();
        
        // let max_width = data_vec.iter().map(|x| format!("{}", x).len()).max().unwrap_or(0) - 7;
        // println!("max_width: {0:?}", max_width);

        println!("{} x {} y", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.i(x, y);
                print!("{:width$}", data_vec[i], width = max_width);
            }
            println!();
        }
    }
}

impl<T> std::ops::Index<usize> for Map2D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Map2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> std::ops::Index<(usize, usize)> for Map2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let i = self.i(index.0, index.1);
        &self.data[i]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Map2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = self.i(index.0, index.1);
        &mut self.data[i]
    }
}


impl<T> std::fmt::Display for Map2D<T> 
where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} x {} y", self.width, self.height)?;

        let max_width = 1 + self.data.iter().map(|x| format!("{}", x).len()).max().unwrap_or(0);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.i(x, y);
                write!(f, "{:width$}", self.data[i], width = max_width)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

