use nonogram_base as nb;

/// Get row clues for the sample small nonogram
///
/// # # _ _
/// # _ _ #
/// # # # _
fn get_small_row_clues() -> Vec<nb::LineClues> {
    vec![vec![2], vec![1, 1], vec![3]]
}

/// Get column clues for the sample small nonogram
///
/// # # _ _
/// # _ _ #
/// # # # _
fn get_small_col_clues() -> Vec<nb::LineClues> {
    vec![vec![3], vec![1, 1], vec![1], vec![1]]
}

/// Get the sample small nonogram
///
/// # # _ _
/// # _ _ #
/// # # # _
fn get_small_nonogram() -> nb::Nonogram {
    let row_clues = get_small_row_clues();
    let col_clues = get_small_col_clues();
    nb::Nonogram::new(row_clues, col_clues)
}

#[test]
fn can_get_dimensions() {
    let non = get_small_nonogram();
    assert_eq!(non.num_rows(), 3);
    assert_eq!(non.num_cols(), 4);
}

#[test]
fn new_is_all_none() {
    let non = get_small_nonogram();
    for row in 0..non.num_rows() {
        for col in 0..non.num_cols() {
            assert_eq!(non.get_tile(row, col), None);
        }
    }
}

#[test]
fn can_set_tile_filled() {
    let mut non = get_small_nonogram();
    let my_row_index = 2;
    let my_col_index = 1;
    let my_tile = nb::Tile::Filled;
    non.set_tile(my_row_index, my_col_index, my_tile);
    for row in 0..non.num_rows() {
        for col in 0..non.num_cols() {
            let act_tile = non.get_tile(row, col);
            let exp_tile = if row == my_row_index && col == my_col_index {
                Some(my_tile)
            } else {
                None
            };
            assert_eq!(act_tile, exp_tile);
        }
    }
}

#[test]
fn can_set_tile_not_filled() {
    let mut non = get_small_nonogram();
    let my_row_index = 2;
    let my_col_index = 1;
    let my_tile = nb::Tile::NotFilled;
    non.set_tile(my_row_index, my_col_index, my_tile);
    for row in 0..non.num_rows() {
        for col in 0..non.num_cols() {
            let act_tile = non.get_tile(row, col);
            let exp_tile = if row == my_row_index && col == my_col_index {
                Some(my_tile)
            } else {
                None
            };
            assert_eq!(act_tile, exp_tile);
        }
    }
}

#[test]
fn can_get_rows() {
    let mut non = get_small_nonogram();
    let empty_row = [None; 4];
    for i in 0..non.num_rows() {
        assert_eq!(non.get_row(i), &empty_row);
    }
    let row_index = 1;
    let col_index = 2;
    let altered_row = [None, None, Some(nb::Tile::Filled), None];
    non.set_tile(row_index, col_index, nb::Tile::Filled);
    for i in 0..non.num_rows() {
        if i == row_index {
            assert_eq!(non.get_row(i), &altered_row);
        } else {
            assert_eq!(non.get_row(i), &empty_row);
        }
    }
}

#[test]
fn can_get_cols() {
    let mut non = get_small_nonogram();
    let empty_col = [None; 3];
    for i in 0..non.num_cols() {
        assert_eq!(non.get_col(i), &empty_col);
    }
    let row_index = 1;
    let col_index = 2;
    let altered_col = [None, Some(nb::Tile::Filled), None];
    non.set_tile(row_index, col_index, nb::Tile::Filled);
    for i in 0..non.num_cols() {
        if i == col_index {
            assert_eq!(non.get_col(i), &altered_col);
        } else {
            assert_eq!(non.get_col(i), &empty_col);
        }
    }
}

#[test]
fn empty_is_not_correct_solution() {
    let non = get_small_nonogram();
    assert!(!non.is_correct_solution());
}
