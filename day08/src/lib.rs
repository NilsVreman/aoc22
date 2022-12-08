use parser;
use std::collections::HashMap;

pub struct Tree {
    height: u32,
}

impl Tree {
    pub fn new(c: &char) -> Tree {
        Tree { height: c.to_digit(10).unwrap() }
    }
}

pub struct Line {
    line: Vec<Tree>,
}

impl Line {
    pub fn new(l: &str) -> Line {
        Line { line: l.chars().map(|x| Tree::new(&x)).collect() }
    }

    /// Visible on left side
    fn l_visible(&self) -> Vec<bool> {
        self.line.iter()
            .enumerate()
            .map(|(i,x)| self.line.iter()
                 .take(i)
                 .find(|other| x.height <= other.height)
                 .is_none())
             .collect()
    }

    /// Visible on right side
    fn r_visible(&self) -> Vec<bool> {
        self.line.iter()
            .rev()
            .enumerate()
            .map(|(i,x)| self.line.iter()
                 .rev()
                 .take(i)
                 .find(|other| x.height <= other.height)
                 .is_none())
             .rev()
             .collect()
    }

    /// Total visible horizontally
    fn h_visible(&self) -> Vec<bool> {
        self.l_visible().iter().zip(&self.r_visible()).map(|(l, r)| *l || *r).collect()
    }

    /// Total number of visible horizontally
    fn nbr_h_visible(&self) -> u32 {
        self.h_visible().iter()
            .filter(|&x| *x)
            .count() as u32
    }

    /// Total view horizontally returned as tuple (left, right)
    fn h_visible_from_tree(&self, c: usize) -> (u32, u32) {
        let r = self.line.iter()
            .skip(c+1)
            .position(|other| self.line[c].height <= other.height)
            .unwrap_or(self.line.len()-2-c)
            + 1;

        let l = self.line.iter()
            .take(c)
            .rev()
            .position(|other| self.line[c].height <= other.height)
            .unwrap_or(c-1)
            + 1;
        (l as u32, r as u32)
    }
}

pub struct Forest {
    grid: Vec<Line>,
}

impl Forest {
    pub fn new(c: &parser::Content) -> Forest {
        Forest { grid: c.content.lines().map(|x| Line::new(&x)).collect() }
    }

    /// Visible on lower (down) side
    fn d_visible(&self, col: usize) -> Vec<bool> {
        self.grid.iter()
            .enumerate()
            .map(|(i,x)| self.grid.iter()
                 .take(i)
                 .find(|other| x.line[col].height <= other.line[col].height)
                 .is_none())
             .collect()
    }

    /// Visible on upper side
    fn u_visible(&self, col: usize) -> Vec<bool> {
        self.grid.iter()
            .rev()
            .enumerate()
            .map(|(i,x)| self.grid.iter()
                 .rev()
                 .take(i)
                 .find(|other| x.line[col].height <= other.line[col].height)
                 .is_none())
             .rev()
             .collect()
    }

    /// Total visible vertically for column col
    fn v_visible(&self, col: usize) -> Vec<bool> {
        self.d_visible(col).iter()
            .zip(&self.u_visible(col))
            .map(|(l, r)| *l || *r)
            .collect()
    }

    /// Map of all visible trees as seen from outside
    pub fn visible(&self) -> HashMap<(usize, usize), bool> {
        let mut map: HashMap<(usize, usize), bool> = HashMap::new();
        let nrows = self.grid.len();
        let ncols = self.grid[0].line.len();

        let rows: Vec<Vec<bool>> = self.grid.iter()
            .map(|x| x.h_visible())
            .collect();
        let cols: Vec<Vec<bool>> = (0..ncols).map(|x| self.v_visible(x))
            .collect();

        for r in 0..nrows {
            for c in 0..ncols {
                map.insert((r,c), rows[r][c] || cols[c][r]);
            }
        }
        map
    }

    /// counts the number of visible trees as seen from outside the grid.
    pub fn nbr_visible(&self) -> u32 {
        self.visible().values()
            .filter(|&x| *x)
            .count() as u32
    }

    /// Total view vertically returned as tuple (left, right)
    fn v_visible_from_tree(&self, r: usize, c: usize) -> (u32, u32) {
        let d = self.grid.iter()
            .skip(r+1)
            .position(|other| self.grid[r].line[c].height <= other.line[c].height)
            .unwrap_or(self.grid.len()-2-r)
            + 1;

        let u = self.grid.iter()
            .take(r)
            .rev()
            .position(|other| self.grid[r].line[c].height <= other.line[c].height)
            .unwrap_or(r-1)
            + 1;
        (d as u32, u as u32)
    }

    /// Score from tree at position (row, col)
    pub fn visible_from_tree(&self, row: usize, col:usize) -> u32 {
        let (l, r) = self.grid[row].h_visible_from_tree(col); 
        let (d, u) = self.v_visible_from_tree(row, col);
        l*r*d*u
    }

    /// Best Score from a tree
    pub fn best_view_from_tree(&self) -> u32 {
        let mut m = 0;
        let mut max = 0;
        for r in 1..self.grid.len()-1 {
            for c in 1..self.grid[0].line.len()-1 {
                m = self.visible_from_tree(r, c);
                if m > max { max = m; }
            }
        }
        max
    }
}


