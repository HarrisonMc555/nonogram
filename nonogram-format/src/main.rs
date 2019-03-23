use nonogram_base as nb;
mod lib;
use lib::NonogramFormatter;

fn main() {
    let row_clues = vec![vec![2], vec![1, 1], vec![3]];
    let col_clues = vec![vec![3], vec![1, 1], vec![1], vec![1]];
    let mut non = nb::Nonogram::new(row_clues, col_clues);
    let formatter = NonogramFormatter::default();
    let lines = formatter.get_lines(&non);
    println!("{}\n", lines.join("\n"));
    non.set_tile(1, 2, nb::Tile::Filled);
    non.set_tile(2, 2, nb::Tile::NotFilled);
    let lines = formatter.get_lines(&non);
    println!("{}", lines.join("\n"));
}
