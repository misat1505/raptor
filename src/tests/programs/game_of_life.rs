use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn game_of_life() {
    let text = BufReader::new(
        r##"
fn print_board(&str[][] board): void {
  for (i64 i = 0; i < vector_size(&board); i += 1) {
    for (i64 j = 0; j < vector_size(&board[0]); j += 1) {
      print(board[i][j]);
    }
    println("");
  }
}
  
fn next_state(&str[][] board): str[][] {
  str[][] next_board = [];

  for (i64 i = 0; i < vector_size(&board); i += 1) {
    str[] row = board[i];
    str[] next_state_row = [];
    for (i64 j = 0; j < vector_size(&row); j += 1) {
      str current_cell = board[i][j];

      i64 alive_neighbours = 0;
      i64 dead_neighbours = 0;

      for (i64 dx = -1; dx <= 1; dx += 1) {
        for (i64 dy = -1; dy <= 1; dy += 1) {
          i64 x = j + dx;
          i64 y = i + dy;
          bool is_x_in_bounds = x >= 0 && x < vector_size(&row);
          bool is_y_in_bounds = y >= 0 && y < vector_size(&board);
          bool is_current_cell = (dx == 0 && dy == 0);
          bool is_valid_neighbour = is_x_in_bounds && is_y_in_bounds && !is_current_cell;
          if (is_valid_neighbour) {
            if (board[y][x] == "#") alive_neighbours += 1;
            else dead_neighbours += 1;
            
          }
        }
      }

      if (current_cell == ".") {
        if (alive_neighbours == 3) vector_push(&next_state_row, "#");
        else vector_push(&next_state_row, ".");
      } else if (current_cell == "#") {
        if (alive_neighbours == 2 || alive_neighbours == 3) vector_push(&next_state_row, "#");
        else vector_push(&next_state_row, ".");
      }
    }

    vector_push(&next_board, next_state_row);
  }

  return next_board;
}

str[][] board = [
  [".", ".", ".", ".", "."],
  [".", ".", "#", ".", "."],
  [".", ".", "#", ".", "."],
  [".", ".", "#", ".", "."],
  [".", ".", ".", ".", "."]
];

board = next_state(&board);
print_board(&board);
    "##
        .as_bytes(),
    );

    assert_same_output(text, ".....\n.....\n.###.\n.....\n.....\n");
}
