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
    FilledWithColor(char),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Nonogram {
    tiles: Array2D<MaybeTile>,
    row_clues: Vec<LineClues>,
    column_clues: Vec<LineClues>,
}

impl Nonogram {
    pub fn new(row_clues: Vec<LineClues>, column_clues: Vec<LineClues>) -> Self {
        let num_rows = row_clues.len();
        let num_cols = column_clues.len();
        let tiles = Array2D::filled_with(None, num_rows, num_cols);
        Nonogram {
            tiles,
            row_clues,
            column_clues,
        }
    }

    pub fn rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = &MaybeTile>> {
        self.tiles.rows_iter()
    }

    pub fn cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = &MaybeTile>> {
        self.tiles.columns_iter()
    }

    pub fn num_rows(&self) -> usize {
        self.row_clues.len()
    }

    pub fn num_cols(&self) -> usize {
        self.column_clues.len()
    }

    pub fn get_row(&self, row: usize) -> impl Iterator<Item = &MaybeTile> {
        self.tiles.row_iter(row)
    }

    pub fn get_column(&self, column: usize) -> impl Iterator<Item = &MaybeTile> {
        self.tiles.column_iter(column)
    }

    pub fn row_clues(&self) -> &[LineClues] {
        &self.row_clues
    }

    pub fn column_clues(&self) -> &[LineClues] {
        &self.column_clues
    }

    pub fn row_clues_at(&self, index: usize) -> &LineClues {
        &self.row_clues[index]
    }

    pub fn column_clues_at(&self, index: usize) -> &LineClues {
        &self.column_clues[index]
    }

    pub fn get_tile(&self, row: usize, column: usize) -> MaybeTile {
        self.tiles[(row, column)]
    }

    pub fn set_tile(&mut self, row: usize, column: usize, tile: Tile) {
        self.tiles[(row, column)] = Some(tile);
    }

    pub fn unset_tile(&mut self, row: usize, column: usize) {
        self.tiles[(row, column)] = None;
    }

    pub fn is_correct_solution(&self) -> bool {
        self.row_clues == self.row_sequence_lengths()
            && self.column_clues == self.column_sequence_lengths()
    }

    fn row_sequence_lengths(&self) -> Vec<LineClues> {
        self.tiles
            .rows_iter()
            .map(|row_iter| Nonogram::sequence_lengths(row_iter))
            .collect()
    }

    fn column_sequence_lengths(&self) -> Vec<LineClues> {
        self.tiles
            .columns_iter()
            .map(|column_iter| Nonogram::sequence_lengths(column_iter))
            .collect()
    }

    fn sequence_lengths<'a, I>(sequence: I) -> LineClues
    where
        I: Iterator<Item = &'a MaybeTile>,
    {
        let sequence =
            sequence.map(|maybe_tile| maybe_tile.unwrap_or(Tile::NotFilled));
        let groups = sequence.group_by(|&t| t);
        let filled = groups.into_iter().filter(|(tile, _)| tile.is_filled());
        filled.map(|(_, group)| group.count()).collect()
    }
}
