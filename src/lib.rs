use itertools::Itertools;

pub type Length = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tile {
    Filled,
    NotFilled,
}

macro_rules! rows {
    ( $nonogram:expr ) => {
        $nonogram
            .row_indices()
            .map(|index| $nonogram.get_row(index))
    };
}

macro_rules! cols {
    ( $nonogram:expr ) => {
        $nonogram
            .col_indices()
            .map(|index| $nonogram.get_col(index))
    };
}

pub struct Nonogram {
    grid_row_major: Vec<Option<Tile>>,
    grid_col_major: Vec<Option<Tile>>,
    row_hints: Vec<Vec<Length>>,
    col_hints: Vec<Vec<Length>>,
}

impl Nonogram {
    pub fn new(
        row_hints: Vec<Vec<Length>>,
        col_hints: Vec<Vec<Length>>,
    ) -> Self {
        let num_rows = row_hints.len();
        let num_cols = col_hints.len();
        let num_tiles = num_rows * num_cols;
        let grid_row_major = vec![None; num_tiles];
        let grid_col_major = vec![None; num_tiles];
        Nonogram {
            grid_row_major,
            grid_col_major,
            row_hints,
            col_hints,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.row_hints.len()
    }

    pub fn num_cols(&self) -> usize {
        self.col_hints.len()
    }

    pub fn get_tile(&self, row: usize, col: usize) -> Option<Tile> {
        let index_row_major = self.index_row_major(row, col);
        self.grid_row_major[index_row_major]
    }

    pub fn set_tile(&mut self, row: usize, col: usize, tile: Tile) {
        let index_row_major = self.index_row_major(row, col);
        self.grid_row_major[index_row_major] = Some(tile);
    }

    pub fn is_filled(&self) -> bool {
        self.grid_row_major.iter().all(Option::is_some)
    }

    pub fn is_valid_solution(&self) -> bool {
        self.row_hints == self.row_sequence_lengths()
            && self.col_hints == self.col_sequence_lengths()
    }

    fn index_row_major(&self, row: usize, col: usize) -> usize {
        row * self.num_cols() + col
    }

    fn index_col_major(&self, row: usize, col: usize) -> usize {
        col * self.num_rows() + row
    }

    fn get_row(&self, row: usize) -> &[Option<Tile>] {
        let start_index = self.index_row_major(row, 0);
        let end_index = self.index_row_major(row + 1, 0);
        &self.grid_row_major[start_index..end_index]
    }

    fn get_col(&self, col: usize) -> &[Option<Tile>] {
        let start_index = self.index_col_major(col, 0);
        let end_index = self.index_col_major(col + 1, 0);
        &self.grid_col_major[start_index..end_index]
    }

    fn row_indices(&self) -> std::ops::Range<usize> {
        0..self.num_rows()
    }

    fn col_indices(&self) -> std::ops::Range<usize> {
        0..self.num_cols()
    }

    fn row_sequence_lengths(&self) -> Vec<Vec<Length>> {
        rows!(self)
            .map(|row| Nonogram::sequence_lengths(row))
            .collect()
    }

    fn col_sequence_lengths(&self) -> Vec<Vec<Length>> {
        cols!(self)
            .map(|col| Nonogram::sequence_lengths(col))
            .collect()
    }

    fn sequence_lengths(sequence: &[Option<Tile>]) -> Vec<Length> {
        let sequence = sequence
            .iter()
            .map(|maybe_tile| maybe_tile.unwrap_or(Tile::NotFilled));
        let groups = sequence.group_by(|&t| t);
        let filled = groups.into_iter().filter(|(key, _)| *key == Tile::Filled);
        filled.map(|(_, group)| group.count() as Length).collect()
    }
}
