use crate::game::Board;
use crate::BOARD_SIZE;
use crate::STACK_AMOUNT;
use crate::STACK_SIZE;

use conv::ValueFrom;

pub struct GetBoardValueParam<'a> {
  board: &'a Board, 
  size_weight: [f64; STACK_SIZE],
  cell_weight: [[f64; BOARD_SIZE]; BOARD_SIZE]
}

pub struct GetStackValueParam<'a> {
  board: &'a Board,
  size_weight: [f64; STACK_SIZE],
}

pub fn get_board_value (param: GetBoardValueParam) -> f64 {
  let GetBoardValueParam { board, size_weight, cell_weight } = param;
  let mut board_value: f64 = 0.0;
  for i in 0..BOARD_SIZE {
    for j in 0..BOARD_SIZE {
      if let Some(player) = board.cells[i][j].owner {
        let cell_value = f64::value_from(size_weight[board.cells[i][j].size-1]).unwrap();
        if player == board.now_player {
          board_value += cell_weight[i][j]*cell_value;
        } else {
          board_value -= cell_weight[i][j]*cell_value;
        }
      }
    }
  }
  board_value
}

pub fn get_stack_value (param: GetStackValueParam) -> f64 {
  let GetStackValueParam { board, size_weight } = param;
  let mut stack_value: f64 = 0.0;
  for i in 0..STACK_SIZE {
    let my_stack:f64 = size_weight[i] * f64::value_from(
      board.stacks[board.now_player.number()][i]
    ).unwrap();
    let op_stack:f64 = size_weight[i] * f64::value_from(
      board.stacks[board.now_player.opponent().number()][i]
    ).unwrap();
    stack_value += my_stack;
    stack_value -= op_stack;
  }
  stack_value
}

// safe, draw preferred player
pub fn heuristic1 (board: &Board) -> f64 {
  assert!(STACK_SIZE == 5);
  assert!(BOARD_SIZE == 3);
  let cell_weight = [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
  let size_weight = [1.0, 2.0, 3.0, 4.0, 5.0];
  let board_value = get_board_value(GetBoardValueParam{ board, size_weight, cell_weight });
  let stack_value = get_stack_value(GetStackValueParam{ board, size_weight });
  let divider: f64 = 100.0;
  (2.0*board_value + stack_value) / divider
}

// aggressive player, somewhat better
pub fn heuristic2 (board: &Board) -> f64 {
  assert!(STACK_SIZE == 5);
  assert!(BOARD_SIZE == 3);
  let cell_weight = [[5.0, 2.0, 5.0], [2.0, 3.0, 2.0], [5.0, 2.0, 5.0]];
  let size_weight = [1.0, 2.0, 3.0, 4.0, 5.0];
  let board_value = get_board_value(GetBoardValueParam{ board, size_weight, cell_weight });
  let stack_value = get_stack_value(GetStackValueParam{ board, size_weight });
  let divider: f64 = 500.0;
  (5.0*board_value + stack_value) / divider
}
