use crate::nonogram::{LineClues, MaybeTile, Nonogram, Tile};
use std::fmt::Display;

type StringGrid = Vec<Vec<String>>;

pub struct Formatter {
    filled_string: String,
    not_filled_string: String,
    none_string: String,
    do_display_numbers: bool,
}

impl Formatter {
    pub fn new(
        filled_string: &str,
        not_filled_string: &str,
        none_string: &str,
        do_display_numbers: bool,
    ) -> Self {
        Formatter {
            filled_string: filled_string.to_string(),
            not_filled_string: not_filled_string.to_string(),
            none_string: none_string.to_string(),
            do_display_numbers,
        }
    }

    pub fn get_string_grid(&self, non: &Nonogram) -> StringGrid {
        let _rows_string_grid = Formatter::get_rows_clue_string_grid(non);
        let _cols_string_grid = Formatter::get_cols_clue_string_grid(non);
        let _cells_string_grid = self.get_cells_string_grid(non);
        vec![]
    }

    pub fn get_string_lines(&self, _non: &Nonogram) -> Vec<String> {
        vec![]
    }

    pub fn get_string(&self, _non: &Nonogram) -> Vec<String> {
        vec![]
    }

    fn get_rows_clue_string_grid(non: &Nonogram) -> StringGrid {
        let max_num_clues = get_max_num_row_clues(non);
        let clue_string_grid = get_string_grid(non.row_clues());
        Formatter::waterfill_clue_string_grid(&clue_string_grid, max_num_clues)
    }

    fn get_cols_clue_string_grid(non: &Nonogram) -> StringGrid {
        let max_num_clues = get_max_num_col_clues(non);
        let clue_string_grid = get_string_grid(non.col_clues());
        Formatter::waterfill_clue_string_grid(&clue_string_grid, max_num_clues)
    }

    fn get_cells_string_grid(&self, non: &Nonogram) -> StringGrid {
        let _ = (0..non.num_rows())
            .map(|row_index| self.get_grid_line(non, row_index))
            .collect();
        vec![]
    }

    fn waterfill_clue_string_grid(
        clue_string_grid: &StringGrid,
        max_num_clues: usize,
    ) -> StringGrid {
        let longest_clue_length = clue_string_grid
            .iter()
            .flat_map(|row| row.iter().map(String::len))
            .max()
            .unwrap();
        let filler_spaces = " ".repeat(longest_clue_length);
        clue_string_grid
            .iter()
            .map(|row_strings| {
                Formatter::get_one_line_clue_string_vec(
                    row_strings,
                    max_num_clues,
                    &filler_spaces,
                )
            })
            .collect()
    }

    fn get_one_line_clue_string_vec(
        clue_strings: &[String],
        max_num_clues: usize,
        filler_spaces: &str,
    ) -> Vec<String> {
        let num_clues = clue_strings.len();
        let num_filler_cells = max_num_clues - num_clues;
        let filler_strings =
            (0..num_filler_cells).map(|_| filler_spaces.to_string());
        let clue_strings = clue_strings.iter().cloned();
        filler_strings.chain(clue_strings).collect()
    }

    pub fn get_lines(&self, non: &Nonogram) -> Vec<String> {
        let grid_lines = self.get_only_grid_lines(non);
        if !self.do_display_numbers {
            return grid_lines;
        }
        let row_lines = self.get_row_clue_lines(non);
        let col_lines = self.get_col_clue_lines(non);
        let max_row_width = row_lines[0].len();
        let max_col_width = col_lines[0].len();
        let leading_spaces = " ".repeat(max_row_width);
        let col_lines_with_leading_spaces = col_lines
            .iter()
            .map(|line| format!("{}  {}", leading_spaces, line));
        let horizontal_line =
            format!("{}  {}", leading_spaces, "_".repeat(max_col_width));
        let row_and_grid_lines = row_lines.iter().zip(grid_lines.iter()).map(
            |(row_line, grid_line)| format!("{} |{}", row_line, grid_line),
        );
        col_lines_with_leading_spaces
            .chain(Some(horizontal_line))
            .chain(row_and_grid_lines)
            .collect()
    }

    pub fn get_only_grid_lines(&self, non: &Nonogram) -> Vec<String> {
        (0..non.num_rows())
            .map(|row_index| self.get_grid_line(non, row_index))
            .collect()
    }

    fn get_grid_line(&self, non: &Nonogram, index: usize) -> String {
        let tile_strings: Vec<_> = non
            .get_row(index)
            .iter()
            .map(|maybe_tile| self.format_tile(*maybe_tile))
            .collect();
        tile_strings.join(" ")
    }

    fn format_tile(&self, maybe_tile: MaybeTile) -> &str {
        match maybe_tile {
            Some(Tile::Filled) => &self.filled_string,
            Some(Tile::NotFilled) => &self.not_filled_string,
            None => &self.none_string,
        }
    }

    fn get_row_clue_lines(&self, non: &Nonogram) -> Vec<String> {
        let max_num_clues = non.row_clues().iter().map(Vec::len).max().unwrap();
        let clue_strings: Vec<Vec<_>> = non
            .row_clues()
            .iter()
            .map(|row| row.iter().map(|clue| clue.to_string()).collect())
            .collect();
        // String::len returns number of bytes, but we're restricting this to
        // the formatted version of a usize, so that's the number of
        // characters.
        let longest_clue_length = clue_strings
            .iter()
            .flat_map(|v| v.iter().map(|s| s.len()))
            .max()
            .unwrap();
        let clue_width = longest_clue_length + 1;
        // let clue_width = longest_clue_length;
        let filler_spaces = " ".repeat(clue_width);
        clue_strings
            .iter()
            .map(|clues| {
                Formatter::format_one_row_clues(
                    clues,
                    max_num_clues,
                    &filler_spaces,
                    longest_clue_length,
                )
            })
            .collect()
    }

    fn format_one_row_clues(
        clues_strings: &[String],
        max_num_clues: usize,
        filler_spaces: &str,
        longest_clue_length: usize,
    ) -> String {
        let num_clues = clues_strings.len();
        let num_filler_spaces = max_num_clues - num_clues;
        let leading_spaces = filler_spaces.repeat(num_filler_spaces);
        let clues_string = clues_strings
            .iter()
            .map(|clue| format!("{:width$}", clue, width = longest_clue_length))
            .collect::<Vec<_>>()
            .join(" ");
        format!("{}{}", leading_spaces, clues_string)
    }

    fn get_col_clue_lines(&self, non: &Nonogram) -> Vec<String> {
        let max_num_clues = non.col_clues().iter().map(Vec::len).max().unwrap();
        let clue_strings: Vec<Vec<_>> = non
            .col_clues()
            .iter()
            .map(|col| col.iter().map(|clue| clue.to_string()).collect())
            .collect();
        // String::len returns number of bytes, but we're restricting this to
        // the formatted version of a usize, so that's the number of
        // characters.
        let longest_clue_length = clue_strings
            .iter()
            .flat_map(|v| v.iter().map(|s| s.len()))
            .max()
            .unwrap();
        // let clue_width = longest_clue_length + 1;
        let clue_width = longest_clue_length;
        let filler_spaces = " ".repeat(clue_width);
        (0..max_num_clues)
            .map(|i| {
                Formatter::format_col_clues_at(
                    &clue_strings,
                    i,
                    max_num_clues,
                    &filler_spaces,
                )
            })
            .collect()
    }

    fn format_col_clues_at(
        col_clue_strings: &[Vec<String>],
        index: usize,
        max_num_clues: usize,
        filler_spaces: &str,
    ) -> String {
        let strings: Vec<_> = col_clue_strings
            .iter()
            .map(|col| {
                if index + col.len() < max_num_clues {
                    return filler_spaces.to_string();
                }
                let i = index + col.len() - max_num_clues;
                col.get(i)
                    .cloned()
                    .unwrap_or_else(|| filler_spaces.to_string())
            })
            .collect();
        strings.join(" ")
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Formatter {
            filled_string: "#".to_string(),
            not_filled_string: "x".to_string(),
            none_string: "_".to_string(),
            do_display_numbers: true,
        }
    }
}

fn get_max_num_row_clues(nonogram: &Nonogram) -> usize {
    get_max_num_clues(nonogram.row_clues())
}

fn get_max_row_clue_width(nonogram: &Nonogram) -> usize {
    get_max_clue_width(nonogram.row_clues())
}

fn get_max_num_col_clues(nonogram: &Nonogram) -> usize {
    get_max_num_clues(nonogram.col_clues())
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
    clues
        .iter()
        .flat_map(|row| row.iter().map(|clue| clue.to_string().len()))
        .max()
        .unwrap()
}

fn get_string_grid<T>(grid: &[Vec<T>]) -> StringGrid
where
    T: Display,
{
    grid.iter()
        .map(|row| row.iter().map(T::to_string).collect())
        .collect()
}
