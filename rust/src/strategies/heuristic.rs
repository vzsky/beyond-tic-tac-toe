use crate::game::Board;
use crate::BOARD_SIZE;
use crate::STACK_AMOUNT;
use crate::STACK_SIZE;

use conv::ValueFrom;

pub fn heuristic1 (board: &Board) -> f64 {
  let mut board_value: f64 = 0.0;
  let mut stack_value: f64 = 0.0;
  for i in 0..BOARD_SIZE {
    for j in 0..BOARD_SIZE {
      let cell_size:f64 = f64::value_from(board.cells[i][j].size).unwrap();
      if let Some(player) = board.cells[i][j].owner {
        if player == board.now_player {
          board_value += cell_size;
        } else {
          board_value -= cell_size;
        }
      }
    }
  }
  for i in 0..STACK_SIZE {
    let my_stack:f64 = f64::value_from(i*board.stacks[board.now_player.number()][i]).unwrap();
    let op_stack:f64 = f64::value_from(i*board.stacks[board.now_player.opponent().number()][i]).unwrap();
    stack_value += my_stack;
    stack_value -= op_stack;
  }

  let divider: f64 = 100.0;
  (2.0*board_value + stack_value) / divider
}

// not really good since cell_value should be in game constants size
pub fn heuristic2 (board: &Board) -> f64 {
  let cell_value = [[1.0, 2.0, 1.0], [2.0, 3.0, 2.0], [1.0, 2.0, 1.0]];
  let mut board_value: f64 = 0.0;
  let mut stack_value: f64 = 0.0;
  for i in 0..BOARD_SIZE {
    for j in 0..BOARD_SIZE {
      let cell_size:f64 = f64::value_from(board.cells[i][j].size).unwrap();
      if let Some(player) = board.cells[i][j].owner {
        if player == board.now_player {
          board_value += cell_value[i][j]*cell_size;
        } else {
          board_value -= cell_value[i][j]*cell_size;
        }
      }
    }
  }
  for i in 0..STACK_SIZE {
    let my_stack:f64 = f64::value_from(i*board.stacks[board.now_player.number()][i]).unwrap();
    let op_stack:f64 = f64::value_from(i*board.stacks[board.now_player.opponent().number()][i]).unwrap();
    stack_value += my_stack;
    stack_value -= op_stack;
  }
  
  let divider: f64 = 600.0;
  (5.0*board_value + stack_value) / divider
}