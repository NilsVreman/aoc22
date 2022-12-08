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

pub struct Forest {
    grid: Vec<Vec<Tree>>,
}

impl Forest {
    pub fn new(c: &parser::Content) -> Forest {
        Forest { grid: c.content.lines()
            .map(|x| x.chars()
                 .map(|y| Tree { height: y.to_digit(10).unwrap() })
                 .collect())
            .collect() }
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || row == self.grid.len()-1 || col == 0 || col == self.grid[0].len()-1 {
            return true
        }
        // from left
        let l = self.grid[row].iter()
            .take(col)
            .find(|x| self.grid[row][col].height <= x.height)
            .is_none();
        // from right
        let r = self.grid[row].iter()
            .skip(col+1)
            .rev()
            .find(|x| self.grid[row][col].height <= x.height)
            .is_none();
        // from up to down
        let d = self.grid.iter()
            .take(row)
            .find(|x| self.grid[row][col].height <= x[col].height)
            .is_none();
        // from down to up
        let u = self.grid.iter()
            .skip(row+1)
            .rev()
            .find(|x| self.grid[row][col].height <= x[col].height)
            .is_none();

        l || r || d || u
    }

    /// Map of all visible trees as seen from outside
    pub fn visible(&self) -> HashMap<(usize, usize), bool> {
        let mut map: HashMap<(usize, usize), bool> = HashMap::new();
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();

        for r in 0..nrows {
            for c in 0..ncols {
                map.insert((r,c), self.is_visible(r, c));
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

    fn tree_view(&self, row: usize, col: usize) -> u32 {
        if row == 0 || row == self.grid.len()-1 || col == 0 || col == self.grid[0].len()-1 {
            return 0
        }
        // to left
        let l = self.grid[row].iter()
            .take(col)
            .rev()
            .position(|x| self.grid[row][col].height <= x.height)
            .unwrap_or(col-1)
            + 1;
        // to right
        let r = self.grid[row].iter()
            .skip(col+1)
            .position(|x| self.grid[row][col].height <= x.height)
            .unwrap_or(self.grid[row].len() - col - 2)
            + 1;
        // downward
        let d = self.grid.iter()
            .skip(row+1)
            .position(|x| self.grid[row][col].height <= x[col].height)
            .unwrap_or(self.grid.len() - row - 2)
            + 1;
        // upwards
        let u = self.grid.iter()
            .take(row)
            .rev()
            .position(|x| self.grid[row][col].height <= x[col].height)
            .unwrap_or(row-1)
            + 1;

        (l * r * d * u) as u32
    }

    /// Map of all visible trees as seen from outside
    pub fn view(&self) -> HashMap<(usize, usize), u32> {
        let mut map: HashMap<(usize, usize), u32> = HashMap::new();
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();

        for r in 0..nrows {
            for c in 0..ncols {
                map.insert((r,c), self.tree_view(r, c));
            }
        }
        map
    }

    pub fn best_view(&self) -> u32 {
        *self.view().values().max().expect("Undescribed error")
    }
}


