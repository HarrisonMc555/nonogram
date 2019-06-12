//! A struct for representing a Nonogram a (also known as Picross) puzzle
//!
//! Quoting from the [Wikipedia page on
//! Nonograms](https://en.wikipedia.org/wiki/Nonogram):
//!
//! > Nonograms, also known as Picross or Griddlers, are picture logic
//! > puzzles in which cells in a grid must be colored or left blank according
//! > to numbers at the side of the grid to reveal a hidden picture. In this
//! > puzzle type, the numbers are a form of discrete tomography that measures
//! > how many unbroken lines of filled-in squares there are in any given row
//! > or column. For example, a clue of "4 8 3" would mean there are sets of
//! > four, eight, and three filled squares, in that order, with at least one
//! > blank square between successive groups.
//!
//! The [Nonogram] struct is created with two `Vec<LineClues>` (or
//! `Vec<Vec<usize>>` without type aliases). These represent the "clues" for
//! the rows and columns, respectively.
//!
//! Each "tile" in the [Nonogram] struct is either unknown ([None]) or known
//! ([Some]). If it is known, it is either [Filled](Tile::Filled) or
//! [NotFilled](Tile::NotFilled) (i.e. black or white, respectively).
//!
//! A [Nonogram] is considered a correct solution if all of the [Filled] tiles
//! form lengths that match both the row and column "clues". Any unknown
//! ([None]) tiles are treated as [NotFilled].

use array2d::Array2D;
use itertools::Itertools;

pub type MaybeTile = Option<Tile>;
pub type Clue = usize;
pub type LineClues = Vec<Clue>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tile {
    Filled,
    FilledColor(char),
    NotFilled,
}

impl Tile {
    pub fn is_filled(&self) -> bool {
        *self != Tile::NotFilled
    }

    pub fn is_not_filled(&self) -> bool {
        *self == Tile::NotFilled
    }
}

macro_rules! rows {
    ( $nonogram:expr ) => {
        Nonogram::row_indices($nonogram).map(|index| $nonogram.get_row(index))
    };
}

macro_rules! cols {
    ( $nonogram:expr ) => {
        Nonogram::col_indices($nonogram).map(|index| $nonogram.get_col(index))
    };
}

pub struct Nonogram {
    tiles: Array2D<MaybeTile>,
    // grid_row_major: Vec<MaybeTile>,
    // grid_col_major: Vec<MaybeTile>,
    row_clues: Vec<LineClues>,
    col_clues: Vec<LineClues>,
}

impl Nonogram {
    pub fn new(row_clues: Vec<LineClues>, col_clues: Vec<LineClues>) -> Self {
        let num_rows = row_clues.len();
        let num_cols = col_clues.len();
        let num_tiles = num_rows * num_cols;
        let tiles = Array2D::filled_with(None, num_rows, num_cols);
        // let grid_row_major = vec![None; num_tiles];
        // let grid_col_major = vec![None; num_tiles];
        Nonogram {
            tiles,
            // grid_row_major,
            // grid_col_major,
            row_clues,
            col_clues,
        }
    }

    // pub fn rows(&self) -> impl Iterator<Item = &[MaybeTile]> {
    //     (0..self.num_rows())
    //         .map(|i| self.get_row(i))
    //         .collect::<Vec<_>>()
    //         .into_iter()
    // }
    pub fn rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = &MaybeTile>> {
        self.tiles.rows_iter()
    }

    // pub fn cols(&self) -> impl Iterator<Item = &[MaybeTile]> {
    //     (0..self.num_cols())
    //         .map(|i| self.get_col(i))
    //         .collect::<Vec<_>>()
    //         .into_iter()
    // }
    pub fn cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = &MaybeTile>> {
        self.tiles.columns_iter()
    }

    pub fn num_rows(&self) -> usize {
        self.row_clues.len()
    }

    pub fn num_cols(&self) -> usize {
        self.col_clues.len()
    }

    // pub fn get_row(&self, row: usize) -> &[MaybeTile] {
    //     let start_index = self.index_row_major(row, 0);
    //     let end_index = self.index_row_major(row + 1, 0);
    //     &self.grid_row_major[start_index..end_index]
    // }
    pub fn get_row(&self, row: usize) -> impl Iterator<Item = &MaybeTile> {
        self.tiles.row_iter(row)
    }

    // pub fn get_col(&self, col: usize) -> &[MaybeTile] {
    //     let start_index = self.index_col_major(0, col);
    //     let end_index = self.index_col_major(0, col + 1);
    //     // eprintln!("start: {}, end: {}", start_index, end_index);
    //     &self.grid_col_major[start_index..end_index]
    // }
    pub fn get_col(&self, col: usize) -> impl Iterator<Item = &MaybeTile> {
        self.tiles.row_iter(col)
    }

    pub fn row_clues(&self) -> &[LineClues] {
        &self.row_clues
    }

    pub fn col_clues(&self) -> &[LineClues] {
        &self.col_clues
    }

    pub fn row_clues_at(&self, index: usize) -> &LineClues {
        &self.row_clues[index]
    }

    pub fn col_clues_at(&self, index: usize) -> &LineClues {
        &self.col_clues[index]
    }

    pub fn get_tile(&self, row: usize, col: usize) -> MaybeTile {
        // let index_row_major = self.index_row_major(row, col);
        // self.grid_row_major[index_row_major]
        self.tiles[(row, col)]
    }

    pub fn set_tile(&mut self, row: usize, col: usize, tile: Tile) {
        // let index_row_major = self.index_row_major(row, col);
        // self.grid_row_major[index_row_major] = Some(tile);
        // let index_col_major = self.index_col_major(row, col);
        // self.grid_col_major[index_col_major] = Some(tile);
        self.tiles[(row, col)] = Some(tile);
    }

    pub fn unset_tile(&mut self, row: usize, col: usize) {
        // let index_row_major = self.index_row_major(row, col);
        // self.grid_row_major[index_row_major] = None;
        // let index_col_major = self.index_col_major(row, col);
        // self.grid_col_major[index_col_major] = None;
        self.tiles[(row, col)] = None;
    }

    pub fn is_correct_solution(&self) -> bool {
        self.row_clues == self.row_sequence_lengths()
            && self.col_clues == self.col_sequence_lengths()
    }

    fn index_row_major(&self, row: usize, col: usize) -> usize {
        row * self.num_cols() + col
    }

    fn index_col_major(&self, row: usize, col: usize) -> usize {
        col * self.num_rows() + row
    }

    fn row_indices(&self) -> std::ops::Range<usize> {
        0..self.num_rows()
    }

    fn col_indices(&self) -> std::ops::Range<usize> {
        0..self.num_cols()
    }

    fn row_sequence_lengths(&self) -> Vec<LineClues> {
        rows!(self)
            .map(|row| Nonogram::sequence_lengths(row))
            .collect()
    }

    fn col_sequence_lengths(&self) -> Vec<LineClues> {
        cols!(self)
            .map(|col| Nonogram::sequence_lengths(col))
            .collect()
    }

    // fn sequence_lengths(sequence: &[MaybeTile]) -> LineClues {
    //     let sequence = sequence
    //         .iter()
    //         .map(|maybe_tile| maybe_tile.unwrap_or(Tile::NotFilled));
    //     let groups = sequence.group_by(|&t| t);
    //     let filled = groups.into_iter().filter(|(tile, _)| tile.is_filled());
    //     filled.map(|(_, group)| group.count()).collect()
    // }
    fn sequence_lengths<'a, I>(sequence: I) -> LineClues
    where
        I: Iterator<Item = &'a MaybeTile>,
    {
        let sequence = sequence
            .map(|maybe_tile| maybe_tile.unwrap_or(Tile::NotFilled));
        let groups = sequence.group_by(|&t| t);
        let filled = groups.into_iter().filter(|(tile, _)| tile.is_filled());
        filled.map(|(_, group)| group.count()).collect()
    }
}
