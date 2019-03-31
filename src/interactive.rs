use crate::nonogram::{Clue, LineClues, MaybeTile, Nonogram, Tile};
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
    const FILLED_STRING: &'static str = "#";
    const NOT_FILLED_STRING: &'static str = "X";
    const UNKNOWN_STRING: &'static str = "_";

    pub fn new(nonogram: Nonogram) -> Self {
        NonogramView {
            max_num_row_clues: get_max_num_row_clues(&nonogram),
            max_row_clue_width: get_max_row_clue_width(&nonogram),
            max_num_col_clues: get_max_num_col_clues(&nonogram),
            max_col_clue_width: get_max_col_clue_width(&nonogram),
            nonogram,
            focus: (0, 0),
        }
    }

    fn draw_all_row_clues(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_rows() {
            self.draw_row_clues(i, printer);
        }
    }

    fn draw_all_col_clues(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_cols() {
            self.draw_col_clues(i, printer);
        }
    }

    fn draw_row_clues(&self, row_index: usize, printer: &Printer) {
        let row = self.nonogram.row_clues_at(row_index);
        let num_blank_spaces = self.max_num_row_clues - row.len();
        let x_offset = num_blank_spaces * self.row_clue_space_width();
        let y_offset = self.max_num_col_clues + 1;
        for (j, clue) in row.iter().enumerate() {
            let x = x_offset + self.row_clue_space_width() * j;
            let y = y_offset + row_index;
            let position = (x, y);
            NonogramView::draw_clue(
                *clue,
                position,
                self.row_clue_space_width(),
                printer,
            );
        }
    }

    fn draw_col_clues(&self, col_index: usize, printer: &Printer) {
        let col = self.nonogram.col_clues_at(col_index);
        let num_blank_spaces = self.max_num_col_clues - col.len();
        let x_offset = self.max_num_row_clues * self.row_clue_space_width();
        let y_offset = num_blank_spaces;
        for (j, clue) in col.iter().enumerate() {
            let x = x_offset + col_index * self.col_clue_space_width();
            let y = y_offset + j;
            let position = (x, y);
            NonogramView::draw_clue(
                *clue,
                position,
                self.col_clue_space_width(),
                printer,
            );
        }
    }

    fn draw_clue(
        clue: Clue,
        position: (usize, usize),
        width: usize,
        printer: &Printer,
    ) {
        let s = format!("{:>width$}", clue, width = width);
        printer.print(position, &s);
    }

    fn draw_grid(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_rows() {
            self.draw_grid_row(i, printer);
        }
    }

    fn draw_grid_row(&self, index: usize, printer: &Printer) {
        for (j, maybe_tile) in self.nonogram.get_row(index).iter().enumerate() {
            let location = (index, j);
            self.draw_tile(*maybe_tile, location, printer);
        }
    }

    fn draw_tile(
        &self,
        tile: MaybeTile,
        location: (usize, usize),
        printer: &Printer,
    ) {
        let (row, col) = location;
        // all row/col clues + 1 for divider
        let x_offset = self.max_num_row_clues * self.row_clue_space_width() + 1;
        let y_offset = self.max_num_col_clues * self.col_clue_space_width() + 1;
        let x = x_offset + NonogramView::get_cell_width() * col;
        let y = y_offset + row;
        let position = (x, y);
        let s = format!(
            "{:<width$}",
            NonogramView::maybe_tile_to_string(tile),
            width = NonogramView::get_max_cell_width()
        );
        printer.print(position, &s);
    }

    fn row_clue_space_width(&self) -> usize {
        self.max_row_clue_width + 1
    }

    fn col_clue_space_width(&self) -> usize {
        self.max_col_clue_width + 1
    }

    fn maybe_tile_to_string(maybe_tile: MaybeTile) -> &'static str {
        match maybe_tile {
            Some(Tile::Filled) => NonogramView::FILLED_STRING,
            Some(Tile::NotFilled) => NonogramView::NOT_FILLED_STRING,
            None => NonogramView::UNKNOWN_STRING,
        }
    }

    fn get_max_string_width(&self, nonogram: &Nonogram) -> usize {
        *[
            get_max_row_clue_width(nonogram),
            get_max_col_clue_width(nonogram),
            NonogramView::get_max_cell_width(),
        ]
        .into_iter()
        .max()
        .unwrap()
    }

    fn get_cell_width() -> usize {
        NonogramView::get_max_cell_width() + 1
    }

    fn get_max_cell_width() -> usize {
        [
            NonogramView::FILLED_STRING,
            NonogramView::NOT_FILLED_STRING,
            NonogramView::UNKNOWN_STRING,
        ]
        .into_iter()
        .map(|s| s.len())
        .max()
        .unwrap()
    }
}

impl View for NonogramView {
    fn draw(&self, printer: &Printer) {
        // for i in 0..self.nonogram.num_rows() {
        //     let row = self.nonogram.get_row(i);
        //     printer.print((0, i), " ");
        //     for (j, maybe_tile) in row.iter().enumerate() {
        //         let text = NonogramView::maybe_tile_to_string(*maybe_tile);
        //         printer.print((j * 2 + 1, i), text);
        //         printer.print((j * 2 + 2, i), " ");
        //     }
        // }
        self.draw_all_row_clues(printer);
        self.draw_all_col_clues(printer);
        self.draw_grid(printer);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let row_clues_width =
            self.max_num_row_clues * self.row_clue_space_width();
        let col_clues_height = self.max_num_col_clues;
        let grid_width =
            self.nonogram.num_cols() * NonogramView::get_cell_width();
        let grid_height = self.nonogram.num_rows();
        // Clues + divider + grid
        let width = row_clues_width + 1 + grid_width;
        let height = col_clues_height + 1 + grid_height;
        eprintln!("max_num_row_clues: {}", self.max_num_row_clues);
        eprintln!("max_num_col_clues: {}", self.max_num_col_clues);
        eprintln!("max_row_clue_width: {}", self.max_row_clue_width);
        eprintln!("max_col_clue_width: {}", self.max_col_clue_width);
        eprintln!("cell_width: {}", NonogramView::get_cell_width());
        eprintln!("width: {}", width);
        eprintln!("height: {}", height);
        (width, height).into()
        // (self.nonogram.num_cols() * 2 + 1, self.nonogram.num_rows()).into()
    }
}

fn get_max_num_row_clues(nonogram: &Nonogram) -> usize {
    get_max_num_clues(nonogram.row_clues())
}

fn get_max_num_col_clues(nonogram: &Nonogram) -> usize {
    get_max_num_clues(nonogram.col_clues())
}

fn get_max_row_clue_width(nonogram: &Nonogram) -> usize {
    get_max_clue_width(nonogram.row_clues())
}

fn get_max_col_clue_width(nonogram: &Nonogram) -> usize {
    get_max_clue_width(nonogram.col_clues())
}

fn get_max_clue_width(clues: &[LineClues]) -> usize {
    clues
        .iter()
        .flat_map(|row| row.iter().map(|clue| clue.to_string().len()))
        .max()
        .unwrap()
}

fn get_max_num_clues(clues: &[LineClues]) -> usize {
    clues.iter().map(|row| row.len()).max().unwrap()
}
