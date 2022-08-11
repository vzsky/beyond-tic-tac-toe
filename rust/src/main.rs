mod game;
mod game_components;
mod strategies {
  pub mod first_action_player;
  pub mod random_player;
  pub mod heuristic_player;
}

mod test;

fn heuristic (board: &game::Board) -> i32 {
  let mut board_value: i32 = 0;
  let mut stack_value: i32 = 0;
  for i in 0..game::BOARD_SIZE {
    for j in 0..game::BOARD_SIZE {
      let cell_size:i32 = (board.cells[i][j].size).try_into().unwrap();
      if let Some(player) = board.cells[i][j].owner {
        if player == board.now_player {
          board_value += cell_size;
        } else {
          board_value -= cell_size;
        }
      }
    }
  }
  for i in 0..game::STACK_SIZE {
    let my_stack:i32 = (i*board.stacks[board.now_player.number()][i]).try_into().unwrap();
    let op_stack:i32 = (i*board.stacks[board.now_player.opponent().number()][i]).try_into().unwrap();
    stack_value += my_stack;
    stack_value -= op_stack;
  }
  // here divider is a very loose upper bound, lower this will help;
  let divider: i32 = (2*(game::BOARD_SIZE*game::BOARD_SIZE)*game::STACK_SIZE + game::STACK_AMOUNT*game::STACK_SIZE).try_into().unwrap();
  (2*board_value + stack_value) / divider
}

fn main () {
  test::test();
  let p1 = strategies::first_action_player::FirstActionPlayer::boxed_new();
  let p2 = strategies::heuristic_player::HeuristicPlayer::boxed_new(&heuristic);
  let mut game = game::Game::new(p1, p2);
  println!("{}", game.run());
} 