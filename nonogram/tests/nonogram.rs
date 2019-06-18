use nonogram as non;

/// Get row clues for the sample small nonogram
///
/// # # _ _
/// # _ _ #
/// # # # _
fn get_small_row_clues() -> Vec<non::nonogram::LineClues> {
    vec![vec![2], vec![1, 1], vec![3]]
}

/// Get column clues for the sample small nonogram
///
/// # # _ _
/// # _ _ #
/// # # # _
fn get_small_column_clues() -> Vec<non::nonogram::LineClues> {
    vec![vec![3], vec![1, 1], vec![1], vec![1]]
}

/// Get the sample small nonogram
///
/// # # _ _
/// # _ _ #
/// # # # _
fn get_small_nonogram() -> non::Nonogram {
    let row_clues = get_small_row_clues();
    let column_clues = get_small_column_clues();
    non::Nonogram::new(row_clues, column_clues)
}

fn get_correct_solution_grid() -> Vec<Vec<non::Tile>> {
    let mut grid = Vec::with_capacity(3);
    const F: non::Tile = non::Tile::Filled;
    const N: non::Tile = non::Tile::NotFilled;
    grid.push(vec![F, F, N, N]);
    grid.push(vec![F, N, N, F]);
    grid.push(vec![F, F, F, N]);
    grid
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
        for column in 0..non.num_cols() {
            assert_eq!(non.get_tile(row, column), None);
        }
    }
}

#[test]
fn can_set_tile_filled() {
    let mut non = get_small_nonogram();
    let my_row_index = 2;
    let my_column_index = 1;
    let my_tile = non::Tile::Filled;
    non.set_tile(my_row_index, my_column_index, my_tile);
    for row in 0..non.num_rows() {
        for column in 0..non.num_cols() {
            let act_tile = non.get_tile(row, column);
            let exp_tile = if row == my_row_index && column == my_column_index {
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
    let my_column_index = 1;
    let my_tile = non::Tile::NotFilled;
    non.set_tile(my_row_index, my_column_index, my_tile);
    for row in 0..non.num_rows() {
        for column in 0..non.num_cols() {
            let act_tile = non.get_tile(row, column);
            let exp_tile = if row == my_row_index && column == my_column_index {
                Some(my_tile)
            } else {
                None
            };
            assert_eq!(act_tile, exp_tile);
        }
    }
}

#[test]
fn can_unset_tile() {
    let mut non = get_small_nonogram();
    let my_row_index = 2;
    let my_column_index = 1;
    let my_tile = non::Tile::Filled;
    non.set_tile(my_row_index, my_column_index, my_tile);
    for row in 0..non.num_rows() {
        for column in 0..non.num_cols() {
            let act_tile = non.get_tile(row, column);
            let exp_tile = if row == my_row_index && column == my_column_index {
                Some(my_tile)
            } else {
                None
            };
            assert_eq!(act_tile, exp_tile);
        }
    }
    non.unset_tile(my_row_index, my_column_index);
    for row in 0..non.num_rows() {
        for column in 0..non.num_cols() {
            assert_eq!(non.get_tile(row, column), None);
        }
    }
}

#[test]
fn can_get_rows() {
    let mut non = get_small_nonogram();
    let empty_row = [None; 4];
    for i in 0..non.num_rows() {
        assert!(non.get_row(i).eq(empty_row.iter()));
    }
    let row_index = 1;
    let column_index = 2;
    let altered_row = [None, None, Some(non::Tile::Filled), None];
    non.set_tile(row_index, column_index, non::Tile::Filled);
    for i in 0..non.num_rows() {
        if i == row_index {
            assert!(non.get_row(i).eq(altered_row.iter()));
        } else {
            assert!(non.get_row(i).eq(empty_row.iter()));
        }
    }
}

#[test]
fn can_get_cols() {
    let mut non = get_small_nonogram();
    let empty_column = [None; 3];
    println!("I am about to fail");
    println!("{:?}", non);

    for i in 0..non.num_cols() {
        println!(
            "non.get_column({}) = {:?}",
            i,
            non.get_column(i).collect::<Vec<_>>()
        );
        assert!(non.get_column(i).eq(empty_column.iter()));
    }
    let row_index = 1;
    let column_index = 2;
    let altered_column = [None, Some(non::Tile::Filled), None];
    non.set_tile(row_index, column_index, non::Tile::Filled);
    for i in 0..non.num_cols() {
        if i == column_index {
            assert!(non.get_column(i).eq(altered_column.iter()));
        } else {
            assert!(non.get_column(i).eq(empty_column.iter()));
        }
    }
}

#[test]
fn empty_nonogram_is_not_correct() {
    let non = get_small_nonogram();
    assert!(!non.is_correct_solution());
}

#[test]
fn partial_solution_is_incorrect() {
    let mut non = get_small_nonogram();
    let solution = get_correct_solution_grid();
    for (row_i, row) in solution.iter().enumerate() {
        for (column_i, tile) in row.iter().enumerate() {
            // Skip every other cell and ensure that this partial solution is
            // not considered correct.
            if (row_i + column_i) % 2 == 0 {
                continue;
            }
            if *tile == non::Tile::Filled {
                non.set_tile(row_i, column_i, non::Tile::Filled);
            }
            assert!(!non.is_correct_solution());
        }
    }
}

#[test]
fn incorrect_solution_is_incorrect() {
    let mut non = get_small_nonogram();
    let solution = get_correct_solution_grid();
    for (row_i, row) in solution.iter().enumerate() {
        for (column_i, _) in row.iter().enumerate() {
            // Fill every other tile. This should be incorrect.
            if (row_i + column_i) % 2 == 0 {
                non.set_tile(row_i, column_i, non::Tile::Filled);
            }
            assert!(!non.is_correct_solution());
        }
    }
}

#[test]
fn correct_solution_is_correct() {
    let mut non = get_small_nonogram();
    let solution = get_correct_solution_grid();
    let last_filled_cell_indices = (2, 2);
    for (row_i, row) in solution.iter().enumerate() {
        for (column_i, tile) in row.iter().enumerate() {
            if *tile == non::Tile::Filled {
                non.set_tile(row_i, column_i, non::Tile::Filled);
            }

            // If we have not yet set all of the filled tiles, then it should
            // not be considered correct. Once we have set all of the filled
            // tiles, it *should* be considered correct (regardless of whether
            // or not the last tiles are unfilled or blank (None)).
            let should_be_correct_solution = (row_i, column_i) >= last_filled_cell_indices;
            assert_eq!(non.is_correct_solution(), should_be_correct_solution);
        }
    }
}

#[test]
fn blank_tiles_do_not_affect_correct_solution() {
    let mut non = get_small_nonogram();
    let solution = get_correct_solution_grid();
    let last_filled_cell_indices = (2, 2);
    for (row_i, row) in solution.iter().enumerate() {
        for (column_i, tile) in row.iter().enumerate() {
            if *tile == non::Tile::Filled {
                non.set_tile(row_i, column_i, non::Tile::Filled);
            } else {
                non.set_tile(row_i, column_i, non::Tile::NotFilled);
            }

            // If we have not yet set all of the filled tiles, then it should
            // not be considered correct. Once we have set all of the filled
            // tiles, it *should* be considered correct (regardless of whether
            // or not the last tiles are unfilled or blank (None)).
            let should_be_correct_solution = (row_i, column_i) >= last_filled_cell_indices;
            assert_eq!(non.is_correct_solution(), should_be_correct_solution);
        }
    }
}
