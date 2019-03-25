use crate::nonogram::{Nonogram, Tile, MaybeTile};
use cursive::view::View;
use cursive::Printer;
use cursive::vec::Vec2;

pub struct NonogramView {
    nonogram: Nonogram,
    focus: (usize, usize),
}

impl NonogramView {
    pub fn new(nonogram: Nonogram) -> Self {
        NonogramView {
            nonogram,
            focus: (0, 0),
        }
    }
}

impl View for NonogramView {
    fn draw(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_rows() {
            let row = self.nonogram.get_row(i);
            printer.print((0, i), " ");
            for (j, maybe_tile) in row.iter().enumerate() {
                let text = maybe_tile_to_string(maybe_tile);
                printer.print((j*2 + 1, i), text);
                printer.print((j*2 + 2, i), " ");
            }
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        (self.nonogram.num_cols()*2 + 1, self.nonogram.num_rows()).into()
    }
}

fn maybe_tile_to_string(maybe_tile: &MaybeTile) -> &'static str {
    match maybe_tile {
        Some(Tile::Filled) => "#",
        Some(Tile::NotFilled) => "X",
        None => "_",
    }
}
