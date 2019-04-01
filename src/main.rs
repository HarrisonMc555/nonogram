pub mod nonogram;
pub use crate::nonogram::{Nonogram, Tile};

#[cfg(not(feature = "interactive"))]
pub mod formatter;
#[cfg(not(feature = "interactive"))]
pub use crate::formatter::Formatter;

#[cfg(feature = "interactive")]
mod interactive;
#[cfg(feature = "interactive")]
use cursive::event::{Event, Key};
#[cfg(feature = "interactive")]
use cursive::traits::*;
#[cfg(feature = "interactive")]
use cursive::views::{Dialog, LinearLayout, OnEventView, Panel};
#[cfg(feature = "interactive")]
use cursive::Cursive;
#[cfg(feature = "interactive")]
use interactive::NonogramView;

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

    // cursive::logger::init();

    siv.add_layer(
        OnEventView::new(
            Dialog::new()
                .title("Nonogram")
                .content(
                    LinearLayout::horizontal().child(Panel::new(
                        NonogramView::new(non).with_id("game"),
                    )),
                )
                .button("Quit game", |s| {
                    s.quit();
                }),
        )
        .on_event(Event::Key(Key::Up), |s| {
            s.find_id::<NonogramView>("game").unwrap().move_focus_up();
        })
        .on_event(Event::Key(Key::Down), |s| {
            s.find_id::<NonogramView>("game").unwrap().move_focus_down();
        })
        .on_event(Event::Key(Key::Left), |s| {
            s.find_id::<NonogramView>("game").unwrap().move_focus_left();
        })
        .on_event(Event::Key(Key::Right), |s| {
            s.find_id::<NonogramView>("game")
                .unwrap()
                .move_focus_right();
        })
        .on_event(Event::Char('z'), |s| {
            let mut non_view = s.find_id::<NonogramView>("game").unwrap();
            non_view.toggle_filled_focused();
            if non_view.is_correct_solution() {
                // eprintln!("Is correct solution!");
                s.add_layer(
                    Dialog::new().title("You won!").button("Ok", |s| s.quit()),
                );
            } else {
                // eprintln!("Not correct solution");
            }
            // s.find_id::<NonogramView>("game")
            //     .unwrap()
            //     .toggle_filled_focused();
        })
        .on_event(Event::Char('x'), |s| {
            s.find_id::<NonogramView>("game")
                .unwrap()
                .toggle_not_filled_focused();
        })
        .on_event(Event::Char('c'), |s| {
            s.find_id::<NonogramView>("game").unwrap().clear_focused();
        }),
    );

    siv.run();
}
