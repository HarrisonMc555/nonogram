use crate::nonogram::{LineClues, MaybeTile, Nonogram, Tile};
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::Printer;

pub struct NonogramView {
    nonogram: Nonogram,
    focus: (usize, usize),
    max_num_row_clues: usize,
    max_row_clue_width: usize,
    max_num_col_clues: usize,
    max_col_clue_width: usize,
}

impl NonogramView {
    pub fn new(nonogram: Nonogram) -> Self {
        NonogramView {
            max_num_row_clues: NonogramView::get_max_num_row_clues(&nonogram),
            max_row_clue_width: NonogramView::get_max_row_clue_width(&nonogram),
            max_num_col_clues: NonogramView::get_max_num_col_clues(&nonogram),
            max_col_clue_width: NonogramView::get_max_col_clue_width(&nonogram),
            nonogram,
            focus: (0, 0),
        }
    }

    fn get_string_grid(&self) -> Vec<Vec<String>> {
        vec![]
    }

    fn get_max_num_row_clues(nonogram: &Nonogram) -> usize {
        NonogramView::get_max_num_clues(nonogram.row_clues())
    }

    fn get_max_row_clue_width(nonogram: &Nonogram) -> usize {
        NonogramView::get_max_clue_width(nonogram.row_clues())
    }

    fn get_max_num_col_clues(nonogram: &Nonogram) -> usize {
        NonogramView::get_max_num_clues(nonogram.col_clues())
    }

    fn get_max_col_clue_width(nonogram: &Nonogram) -> usize {
        NonogramView::get_max_clue_width(nonogram.col_clues())
    }

    fn get_max_clue_width(clues: &[LineClues]) -> usize {
        clues
            .iter()
            .flat_map(|row| row.iter().map(|clue| clue.to_string().len()))
            .max()
            .unwrap()
    }

    fn get_max_num_clues(clues: &[LineClues]) -> usize {
        clues
            .iter()
            .flat_map(|row| row.iter().map(|clue| clue.to_string().len()))
            .max()
            .unwrap()
    }
}

impl View for NonogramView {
    fn draw(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_rows() {
            let row = self.nonogram.get_row(i);
            printer.print((0, i), " ");
            for (j, maybe_tile) in row.iter().enumerate() {
                let text = maybe_tile_to_string(maybe_tile);
                printer.print((j * 2 + 1, i), text);
                printer.print((j * 2 + 2, i), " ");
            }
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        (self.nonogram.num_cols() * 2 + 1, self.nonogram.num_rows()).into()
    }
}

fn maybe_tile_to_string(maybe_tile: &MaybeTile) -> &'static str {
    match maybe_tile {
        Some(Tile::Filled) => "#",
        Some(Tile::NotFilled) => "X",
        None => "_",
    }
}
