uqse parser;
use queues::*;

pub struct Grid {
    grid: Vec<Vec<isize>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Grid {
    pub fn new(c: &parser::Content) -> Grid {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut row = 0;
        let mut col = 0;
        let map: Vec<Vec<isize>> = c.content.lines()
            .map(|l| {
                row += 1;
                col = 0;
                l.chars().map(|cha| {
                    col += 1;
                    if cha == 'S' {
                        start = (row-1, col-1);
                        0
                    } else if cha == 'E' {
                        end = (row-1, col-1);
                        25
                    } else {
                        cha as isize - 'a' as isize
                    }
                }).collect()
            }).collect();
        Grid { grid: map, start, end }
    }

    pub fn feasible_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        let m = self.grid[row][col] - 1;
        if row >= 1                   && self.grid[row-1][col] >= m { res.push((row-1, col)); }
        if row+1 < self.grid.len()    && self.grid[row+1][col] >= m { res.push((row+1, col)); }
        if col >= 1                   && self.grid[row][col-1] >= m { res.push((row, col-1)); }
        if col+1 < self.grid[0].len() && self.grid[row][col+1] >= m { res.push((row, col+1)); }

        res
    }

    pub fn shortest_path(&self) -> Vec<Vec<usize>> {
        let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; self.grid[0].len()]; self.grid.len()];
        let mut queue = queue![self.end];
        dist[self.end.0][self.end.1] = 0;

        while queue.size() > 0 {
            let (r, c) = queue.remove().expect("Trying to pop empty queue");

            for n in self.feasible_neighbours(r, c) {
                if dist[n.0][n.1] == usize::MAX {
                    queue.add(n).unwrap();
                    dist[n.0][n.1] = dist[r][c] + 1;
                } else if dist[n.0][n.1] > dist[r][c] + 1 {
                    dist[n.0][n.1] = dist[r][c] + 1;
                }
            }
        }

        dist
    }

    pub fn shortest_scenic_path(&self) -> usize {
        let mut dist = self.shortest_path();
        let mut res: Vec<usize> = Vec::new();
        self.grid.iter()
            .enumerate()
            .for_each(|(row, l)| l.iter()
                    .enumerate()
                    .for_each(|(col, &c)| if c == 0 {
                        res.push(dist[row][col]);
                    }));
        *res.iter().min().expect("No value that can be minimal in res")
    }
}
