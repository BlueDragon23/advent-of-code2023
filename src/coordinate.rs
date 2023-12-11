use itertools::Itertools;
use num::{range_inclusive, PrimInt};
use std::cmp::{max, min};
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Coordinate<T: PrimInt> {
    pub row: T,
    pub col: T,
}

pub type PosCoordinate = Coordinate<u64>;

pub type IndexingCoordinate = Coordinate<usize>;

impl IndexingCoordinate {
    pub fn get<V: Copy>(&self, matrix: &[Vec<V>]) -> V {
        matrix[self.row][self.col]
    }
}

impl<T: PrimInt> From<(T, T)> for Coordinate<T> {
    fn from((row, col): (T, T)) -> Self {
        Coordinate { row, col }
    }
}

impl<T: PrimInt> Coordinate<T> {
    pub fn new(row: T, col: T) -> Coordinate<T> {
        Coordinate { row, col }
    }

    pub fn get_between(&self, other: &Coordinate<T>) -> Vec<Coordinate<T>> {
        if self.row == other.row {
            range_inclusive(min(self.col, other.col), max(self.col, other.col))
                .map(|col| Coordinate {
                    row: other.row,
                    col,
                })
                .collect_vec()
        } else if self.col == other.col {
            range_inclusive(min(self.row, other.row), max(self.row, other.row))
                .map(|row| Coordinate { row, col: self.col })
                .collect_vec()
        } else {
            panic!("Invalid coordinates passed, must form a straight line");
        }
    }

    pub fn transpose(&self) -> Coordinate<T> {
        Coordinate {
            row: self.col,
            col: self.row,
        }
    }

    pub fn get_adjacent_points(&self, max_row: T, max_col: T) -> Vec<Coordinate<T>> {
        let mut adj = vec![];
        let min_row = T::min_value();
        let min_col = T::min_value();
        let one = T::one();
        if self.row != min_row {
            adj.push(Coordinate {
                row: self.row - one,
                col: self.col,
            });
        }
        if self.row != max_row - one {
            adj.push(Coordinate {
                row: self.row + one,
                col: self.col,
            });
        }
        if self.col != min_col {
            adj.push(Coordinate {
                row: self.row,
                col: self.col - one,
            });
        }
        if self.col != max_col - one {
            adj.push(Coordinate {
                row: self.row,
                col: self.col + one,
            });
        }
        adj
    }

    pub fn get_adjacent_points_diagonal(&self, max_row: T, max_col: T) -> Vec<Coordinate<T>> {
        let mut adj = self.get_adjacent_points(max_row, max_col);
        let min_row = T::min_value();
        let min_col = T::min_value();
        let one = T::one();
        if self.row != min_row && self.col != min_col {
            adj.push(Coordinate {
                row: self.row - one,
                col: self.col - one,
            });
        }
        if self.row != max_row - one && self.col != max_col - one {
            adj.push(Coordinate {
                row: self.row + one,
                col: self.col + one,
            });
        }
        if self.col != min_col && self.row != max_row - one {
            adj.push(Coordinate {
                row: self.row + one,
                col: self.col - one,
            });
        }
        if self.col != max_col - one && self.row != min_row {
            adj.push(Coordinate {
                row: self.row - one,
                col: self.col + one,
            });
        }
        adj
    }
}

impl<T: PrimInt> Add for Coordinate<T> {
    type Output = Coordinate<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl<T: PrimInt> Sub for Coordinate<T> {
    type Output = Coordinate<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

// Print the matrix of coordinates. Find the min/max row/col in the matrix.
// Between those coordinates, print # if the coordinate is present and . if its absent
pub fn print_coordinates<T: PrimInt>(matrix: &[Coordinate<T>], origin_top_left: bool) {
    let min_row = matrix.iter().map(|c| c.row).min().unwrap();
    let max_row = matrix.iter().map(|c| c.row).max().unwrap();
    let min_col = matrix.iter().map(|c| c.col).min().unwrap();
    let max_col = matrix.iter().map(|c| c.col).max().unwrap();
    let row_iter = if origin_top_left {
        range_inclusive(min_row, max_row).collect_vec()
    } else {
        range_inclusive(min_row, max_row)
            .collect_vec()
            .into_iter()
            .rev()
            .collect_vec()
    };
    for row in row_iter {
        for col in range_inclusive(min_col, max_col) {
            let c = Coordinate { row, col };
            if matrix.contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing_coordinate_1() -> color_eyre::Result<()> {
        let coord = IndexingCoordinate { row: 0, col: 1 };
        let matrix = vec![vec![0, 1], vec![2, 3]];
        let result = coord.get(&matrix);
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_indexing_coordinate_2() -> color_eyre::Result<()> {
        let coord = IndexingCoordinate { row: 1, col: 0 };
        let matrix = vec![vec![0, 1], vec![2, 3]];
        let result = coord.get(&matrix);
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn get_between_col_forward() -> color_eyre::Result<()> {
        let start_coord = Coordinate { row: 0, col: 0 };
        let end_coord = Coordinate { row: 5, col: 0 };
        let between = start_coord.get_between(&end_coord);
        assert_eq!(
            between,
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
                .into_iter()
                .map(Coordinate::from)
                .collect_vec()
        );
        Ok(())
    }

    #[test]
    fn get_between_col_reverse() -> color_eyre::Result<()> {
        let start_coord = Coordinate { row: 5, col: 0 };
        let end_coord = Coordinate { row: 0, col: 0 };
        let between = start_coord.get_between(&end_coord);
        assert_eq!(
            between,
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
                .into_iter()
                .map(Coordinate::from)
                .collect_vec()
        );
        Ok(())
    }

    #[test]
    fn get_between_row_forward() -> color_eyre::Result<()> {
        let start_coord = Coordinate { row: 0, col: 0 };
        let end_coord = Coordinate { row: 0, col: 5 };
        let between = start_coord.get_between(&end_coord);
        assert_eq!(
            between,
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]
                .into_iter()
                .map(Coordinate::from)
                .collect_vec()
        );
        Ok(())
    }

    #[test]
    fn get_between_row_reverse() -> color_eyre::Result<()> {
        let start_coord = Coordinate { row: 0, col: 5 };
        let end_coord = Coordinate { row: 0, col: 0 };
        let between = start_coord.get_between(&end_coord);
        assert_eq!(
            between,
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]
                .into_iter()
                .map(Coordinate::from)
                .collect_vec()
        );
        Ok(())
    }
}
