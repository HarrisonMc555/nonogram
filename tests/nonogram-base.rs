use nonogram_base as nb;

fn get_small_row_hints() -> Vec<Vec<nb::Length>> {
    vec![vec![2], vec![1, 1], vec![3]]
}

fn get_small_col_hints() -> Vec<Vec<nb::Length>> {
    vec![vec![3], vec![1, 1], vec![1], vec![1]]
}

fn get_small_nonogram() -> nb::Nonogram {
    let row_hints = get_small_row_hints();
    let col_hints = get_small_col_hints();
    nb::Nonogram::new(row_hints, col_hints)
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
