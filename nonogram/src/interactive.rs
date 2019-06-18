use crate::nonogram::{Clue, LineClues, MaybeTile, Nonogram, Tile};
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::views::{Dialog, LinearLayout, OnEventView, Panel, ViewRef};
use cursive::Cursive;
use cursive::Printer;

pub fn main() {
    #[rustfmt::skip]
    let row_clues = vec![
        vec![2],
        vec![2, 1],
        vec![1, 1],
        vec![3],
        vec![1, 1],
        vec![1, 1],
        vec![2],
        vec![1, 1],
        vec![1, 2],
        vec![2],
    ];
    #[rustfmt::skip]
    let column_clues = vec![
        vec![2, 1],
        vec![2, 1, 3],
        vec![7],
        vec![1, 3],
        vec![2, 1],
    ];
    let mut non = Nonogram::new(row_clues, column_clues);
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
            get_nonogram_view(s).move_focus_up();
        })
        .on_event(Event::Key(Key::Down), |s| {
            get_nonogram_view(s).move_focus_down();
        })
        .on_event(Event::Key(Key::Left), |s| {
            get_nonogram_view(s).move_focus_left();
        })
        .on_event(Event::Key(Key::Right), |s| {
            get_nonogram_view(s).move_focus_right();
        })
        .on_event(Event::Char('z'), |s| {
            let mut non_view = get_nonogram_view(s);
            non_view.toggle_filled_focused();
            if non_view.is_correct_solution() {
                s.add_layer(
                    Dialog::new().title("You won!").button("Ok", |s| s.quit()),
                );
            }
        })
        .on_event(Event::Char('x'), |s| {
            get_nonogram_view(s).toggle_not_filled_focused();
        })
        .on_event(Event::Char('c'), |s| {
            get_nonogram_view(s).clear_focused();
        }),
    );

    siv.run();
}

fn get_nonogram_view(s: &mut Cursive) -> ViewRef<NonogramView> {
    s.find_id::<NonogramView>("game").unwrap()
}

pub struct NonogramView {
    nonogram: Nonogram,
    focus: (usize, usize),
    max_num_row_clues: usize,
    max_row_clue_width: usize,
    max_num_column_clues: usize,
    max_column_clue_width: usize,
}

impl NonogramView {
    const FILLED_STRING: &'static str = "▣";
    const NOT_FILLED_STRING: &'static str = "☒";
    const UNKNOWN_STRING: &'static str = "☐";
    // const FILLED_STRING: &'static str = "#";
    // const NOT_FILLED_STRING: &'static str = "X";
    // const UNKNOWN_STRING: &'static str = "_";
    const TOP_DIVIDER: char = '─';
    const SIDE_DIVIDER: char = '│';
    const CORNER_DIVIDER: char = '┌';

    pub fn new(nonogram: Nonogram) -> Self {
        NonogramView {
            max_num_row_clues: get_max_num_row_clues(&nonogram),
            max_row_clue_width: get_max_row_clue_width(&nonogram),
            max_num_column_clues: get_max_num_column_clues(&nonogram),
            max_column_clue_width: get_max_column_clue_width(&nonogram),
            nonogram,
            focus: (0, 0),
        }
    }

    pub fn toggle_filled_focused(&mut self) {
        if let Some(Tile::Filled) = self.get_focused() {
            self.clear_focused();
        } else {
            self.set_focused(Tile::Filled);
        }
    }

    pub fn toggle_not_filled_focused(&mut self) {
        if let Some(Tile::NotFilled) = self.get_focused() {
            self.clear_focused();
        } else {
            self.set_focused(Tile::NotFilled);
        }
    }

    pub fn clear_focused(&mut self) {
        let (row, column) = self.focus;
        self.nonogram.unset_tile(row, column);
    }

    pub fn move_focus_up(&mut self) {
        let (mut row, column) = self.focus;
        if row > 0 {
            row -= 1;
        }
        self.focus = (row, column);
    }

    pub fn move_focus_down(&mut self) {
        let (mut row, column) = self.focus;
        if row < self.nonogram.num_rows() - 1 {
            row += 1;
        }
        self.focus = (row, column);
    }

    pub fn move_focus_left(&mut self) {
        let (row, mut column) = self.focus;
        if column > 0 {
            column -= 1;
        }
        self.focus = (row, column);
    }

    pub fn move_focus_right(&mut self) {
        let (row, mut column) = self.focus;
        if column < self.nonogram.num_cols() - 1 {
            column += 1;
        }
        self.focus = (row, column);
    }

    pub fn is_correct_solution(&self) -> bool {
        self.nonogram.is_correct_solution()
    }

    fn get_focused(&self) -> MaybeTile {
        let (row, column) = self.focus;
        self.nonogram.get_tile(row, column)
    }

    fn set_focused(&mut self, tile: Tile) {
        let (row, column) = self.focus;
        self.nonogram.set_tile(row, column, tile);
    }

    fn draw_all_row_clues(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_rows() {
            self.draw_row_clues(i, printer);
        }
    }

    fn draw_all_column_clues(&self, printer: &Printer) {
        for i in 0..self.nonogram.num_cols() {
            self.draw_column_clues(i, printer);
        }
    }

    fn draw_row_clues(&self, row_index: usize, printer: &Printer) {
        let row = self.nonogram.row_clues_at(row_index);
        let num_blank_spaces = self.max_num_row_clues - row.len();
        let x_offset = num_blank_spaces * self.row_clue_space_width();
        let y_offset = self.max_num_column_clues + 1;
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

    fn draw_column_clues(&self, column_index: usize, printer: &Printer) {
        let column = self.nonogram.column_clues_at(column_index);
        let num_blank_spaces = self.max_num_column_clues - column.len();
        let x_offset = self.max_num_row_clues * self.row_clue_space_width();
        let y_offset = num_blank_spaces;
        for (j, clue) in column.iter().enumerate() {
            let x = x_offset + column_index * self.column_clue_space_width();
            let y = y_offset + j;
            let position = (x, y);
            NonogramView::draw_clue(
                *clue,
                position,
                self.column_clue_space_width(),
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
        for (j, maybe_tile) in self.nonogram.get_row(index).enumerate() {
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
        let (row, column) = location;
        // all row/column clues + 1 for divider
        let x_offset = self.max_num_row_clues * self.row_clue_space_width() + 1;
        let y_offset = self.max_num_column_clues + 1;
        let x = x_offset + NonogramView::cell_width() * column;
        let y = y_offset + row;
        let position = (x, y);
        // eprintln!("position: {:?}", position);
        let s = format!(
            "{:<width$}",
            NonogramView::maybe_tile_to_string(tile),
            width = NonogramView::get_max_cell_width()
        );
        if location == self.focus {
            printer.with_color(cursive::theme::ColorStyle::highlight(), |p| {
                p.print(position, &s)
            });
        } else {
            printer.print(position, &s);
        }
    }

    fn draw_borders(&self, printer: &Printer) {
        self.draw_top_border(printer);
        self.draw_side_border(printer);
        self.draw_corner_border(printer);
    }

    fn draw_top_border(&self, printer: &Printer) {
        let x = self.max_num_row_clues * self.row_clue_space_width() + 1;
        let y = self.max_num_column_clues;
        let position = (x, y);
        let width = self.nonogram.num_cols() * NonogramView::cell_width();
        let s = NonogramView::TOP_DIVIDER.to_string().repeat(width);
        printer.print(position, &s);
    }

    fn draw_side_border(&self, printer: &Printer) {
        let x = self.max_num_row_clues * self.row_clue_space_width();
        let y_offset = self.max_num_column_clues + 1;
        let s = NonogramView::SIDE_DIVIDER.to_string();
        // eprintln!("s: {}", s);
        for j in 0..self.nonogram.num_rows() {
            let y = y_offset + j;
            let position = (x, y);
            // eprintln!("position: {:?}", position);
            printer.print(position, &s);
        }
    }

    fn draw_corner_border(&self, printer: &Printer) {
        let x = self.max_num_row_clues * self.row_clue_space_width();
        let y = self.max_num_column_clues;
        let position = (x, y);
        let s = NonogramView::CORNER_DIVIDER.to_string();
        printer.print(position, &s);
    }

    fn row_clue_space_width(&self) -> usize {
        self.max_row_clue_width + 1
    }

    fn column_clue_space_width(&self) -> usize {
        self.max_column_clue_width + 1
    }

    fn maybe_tile_to_string(maybe_tile: MaybeTile) -> &'static str {
        match maybe_tile {
            Some(Tile::Filled) => NonogramView::FILLED_STRING,
            Some(Tile::FilledWithColor(_)) => NonogramView::FILLED_STRING,
            Some(Tile::NotFilled) => NonogramView::NOT_FILLED_STRING,
            None => NonogramView::UNKNOWN_STRING,
        }
    }

    #[allow(dead_code)]
    fn get_max_string_width(&self, nonogram: &Nonogram) -> usize {
        *[
            get_max_row_clue_width(nonogram),
            get_max_column_clue_width(nonogram),
            NonogramView::get_max_cell_width(),
        ]
        .into_iter()
        .max()
        .unwrap()
    }

    fn cell_width() -> usize {
        NonogramView::get_max_cell_width() + 1
    }

    fn get_max_cell_width() -> usize {
        1
        // 2
        // [
        //     NonogramView::FILLED_STRING,
        //     NonogramView::NOT_FILLED_STRING,
        //     NonogramView::UNKNOWN_STRING,
        // ]
        // .into_iter()
        // .map(|s| s.len())
        // .max()
        // .unwrap()
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
        self.draw_all_column_clues(printer);
        self.draw_borders(printer);
        self.draw_grid(printer);
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let row_clues_width =
            self.max_num_row_clues * self.row_clue_space_width();
        let column_clues_height = self.max_num_column_clues;
        let grid_width = self.nonogram.num_cols() * NonogramView::cell_width();
        let grid_height = self.nonogram.num_rows();
        // Clues + divider + grid
        let width = row_clues_width + 1 + grid_width;
        let height = column_clues_height + 1 + grid_height;
        // eprintln!("max_num_row_clues: {}", self.max_num_row_clues);
        // eprintln!("max_num_column_clues: {}", self.max_num_column_clues);
        // eprintln!("max_row_clue_width: {}", self.max_row_clue_width);
        // eprintln!("max_column_clue_width: {}", self.max_column_clue_width);
        // eprintln!("cell_width: {}", NonogramView::cell_width());
        // eprintln!("width: {}", width);
        // eprintln!("height: {}", height);
        (width, height).into()
        // (self.nonogram.num_cols() * 2 + 1, self.nonogram.num_rows()).into()
    }
}

fn get_max_num_row_clues(nonogram: &Nonogram) -> usize {
    get_max_num_clues(nonogram.row_clues())
}

fn get_max_num_column_clues(nonogram: &Nonogram) -> usize {
    get_max_num_clues(nonogram.column_clues())
}

fn get_max_row_clue_width(nonogram: &Nonogram) -> usize {
    get_max_clue_width(nonogram.row_clues())
}

fn get_max_column_clue_width(nonogram: &Nonogram) -> usize {
    get_max_clue_width(nonogram.column_clues())
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
