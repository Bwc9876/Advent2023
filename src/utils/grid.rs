use super::{
    dir::{Movement, CARDINALS},
    Direction,
};

pub type Position = (usize, usize);

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let width = data[0].len();
        let height = data.len();
        Self {
            width,
            height,
            data,
        }
    }

    pub fn parse(input: &str) -> Self
    where
        T: From<char>,
    {
        let data: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Self {
            width: data[0].len(),
            height: data.len(),
            data,
        }
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        let (x, y) = pos;
        self.data.get(y).and_then(|row| row.get(x))
    }

    // pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
    //     let (x, y) = pos;
    //     self.data.get_mut(y).and_then(|row| row.get_mut(x))
    // }

    pub fn iter(&self) -> FullGridIter<T> {
        FullGridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn iter_rows(&self) -> GridRowIter<T> {
        GridRowIter { grid: self, y: 0 }
    }

    pub fn iter_cols(&self) -> GridColIter<T> {
        GridColIter { grid: self, x: 0 }
    }

    pub fn get_row(&self, y: usize) -> Option<Vec<&T>> {
        self.data.get(y).map(|row| row.iter().collect())
    }

    // pub fn get_col(&self, x: usize) -> Option<Vec<&T>> {
    //     self.data.iter().map(|row| row.get(x)).collect()
    // }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn check_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.0 < self.width as isize && pos.1 >= 0 && pos.1 < self.height as isize
    }

    pub fn get_next_pos(&self, current_pos: Position, dir: impl Movement) -> Option<Position> {
        self.get_next_pos_times(current_pos, dir, 1)
    }

    pub fn get_next_pos_times(
        &self,
        current_pos: Position,
        dir: impl Movement,
        times: usize,
    ) -> Option<Position> {
        let new_pos = dir.add_to_pos_times(current_pos, times as isize);

        if self.check_bounds(new_pos) {
            Some((new_pos.0 as usize, new_pos.1 as usize))
        } else {
            None
        }
    }

    pub fn get_direct_adjacents(&self, pos: Position) -> Vec<(Position, Direction)> {
        CARDINALS
            .iter()
            .filter_map(|dir| {
                self.get_next_pos(pos, *dir)
                    .map(|next_pos| (next_pos, *dir))
            })
            .collect()
    }

    // pub fn resize(&mut self, new_width: usize, new_height: usize, default: T)
    // where
    //     T: Clone,
    // {
    //     if new_width > self.width {
    //         for row in self.data.iter_mut() {
    //             row.resize(new_width, default.clone());
    //         }
    //     }
    //     self.data.resize(new_height, vec![default; new_width]);
    //     self.width = new_width;
    //     self.height = new_height;
    // }

    // pub fn get_adjacents_with_diag(&self, pos: Position) -> Vec<(Position, ExpandedDirection)> {
    //     COMPASS
    //         .iter()
    //         .filter_map(|dir| {
    //             self.get_next_pos(pos, *dir)
    //                 .map(|next_pos| (next_pos, *dir))
    //         })
    //         .collect()
    // }
}

pub struct FullGridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for FullGridIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.height {
            let pos = (self.x, self.y);
            let item = self.grid.get(pos).unwrap();
            self.x += 1;
            if self.x >= self.grid.width {
                self.x = 0;
                self.y += 1;
            }
            Some((pos, item))
        } else {
            None
        }
    }
}

pub struct GridRowIter<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.height {
            let row = self.grid.data[self.y].iter().collect();
            self.y += 1;
            Some(row)
        } else {
            None
        }
    }
}

pub struct GridColIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
}

impl<'a, T> Iterator for GridColIter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.grid.width {
            let col = self.grid.data.iter().map(|row| &row[self.x]).collect();
            self.x += 1;
            Some(col)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GridPointer {
    pub pos: Position,
    pub dir: Direction,
}

impl GridPointer {
    pub fn zero() -> Self {
        Self {
            pos: (0, 0),
            dir: Direction::East,
        }
    }

    pub fn new(pos: Position, dir: Direction) -> Self {
        Self { pos, dir }
    }

    pub fn get_next_pos<T>(&self, grid: &Grid<T>) -> Option<Position> {
        grid.get_next_pos(self.pos, self.dir)
    }

    pub fn move_to_next<T>(&mut self, grid: &Grid<T>) -> bool {
        if let Some(next_pos) = self.get_next_pos(grid) {
            self.pos = next_pos;
            true
        } else {
            false
        }
    }

    // pub fn reverse(&mut self) {
    //     self.dir = self.dir.opposite();
    // }

    // pub fn turn(&mut self, clockwise: bool) {
    //     self.dir = self.dir.ninety_deg(clockwise);
    // }
}
