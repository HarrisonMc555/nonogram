pub mod nonogram;
pub mod formatter;
pub use crate::nonogram::{Nonogram, Tile};
pub use crate::formatter::Formatter;

fn main() {
    let row_clues = vec![vec![2], vec![1, 1], vec![3]];
    let col_clues = vec![vec![3], vec![1, 1], vec![1], vec![1]];
    let mut non = Nonogram::new(row_clues, col_clues);
    let formatter = Formatter::default();
    let lines = formatter.get_lines(&non);
    println!("{}\n", lines.join("\n"));

    non.set_tile(1, 2, Tile::Filled);
    non.set_tile(2, 2, Tile::NotFilled);
    let formatter = Formatter::new("#", "X", "_", true);
    let lines = formatter.get_lines(&non);
    println!("{}", lines.join("\n"));
}
