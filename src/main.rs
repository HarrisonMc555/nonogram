pub mod nonogram;
pub use crate::nonogram::{Nonogram, Tile};

pub mod formatter;
pub use crate::formatter::Formatter;

#[cfg(feature = "interactive")]
mod interactive;
#[cfg(feature = "interactive")]
use interactive::NonogramView;
#[cfg(feature = "interactive")]
use cursive::Cursive;
#[cfg(feature = "interactive")]
use cursive::views::{Dialog, LinearLayout, Panel};

#[cfg(not(feature = "interactive"))]
fn main() {
    let row_clues = vec![vec![2], vec![1, 1], vec![3]];
    let col_clues = vec![vec![3], vec![1, 1], vec![1], vec![1]];
    let mut non = Nonogram::new(row_clues, col_clues);
    let formatter = Formatter::default();
    let string_grid = formatter.get_string_grid(&non);
    for row in string_grid {
        println!("{}", row.join(" "));
    }
    // let lines = formatter.get_lines(&non);
    // println!("{}\n", lines.join("\n"));

    non.set_tile(1, 2, Tile::Filled);
    non.set_tile(2, 2, Tile::NotFilled);
    let formatter = Formatter::new("#", "X", "_", true);
    let string_grid = formatter.get_string_grid(&non);
    for row in string_grid {
        println!("{}", row.join(" "));
    }
    // let lines = formatter.get_lines(&non);
    // println!("{}", lines.join("\n"));
}

#[cfg(feature = "interactive")]
fn main() {
    let row_clues = vec![vec![2], vec![1, 1], vec![3]];
    let col_clues = vec![vec![3], vec![1, 1], vec![1], vec![1]];
    let mut non = Nonogram::new(row_clues, col_clues);
    non.set_tile(1, 2, Tile::Filled);
    non.set_tile(2, 2, Tile::NotFilled);

    let mut siv = Cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("Nonogram")
            .content(LinearLayout::horizontal()
                     .child(Panel::new(NonogramView::new(non)))
            )
            .button("Quit game", |s| {
                s.quit();
            })
    );

    siv.run();
}
