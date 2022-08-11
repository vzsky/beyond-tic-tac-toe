pub const BOARD_SIZE:usize = 3;
pub const STACK_AMOUNT:usize = 1;
pub const STACK_SIZE:usize = 5;

mod game;
mod game_components;
mod strategies {
  pub mod first_action_player;
  pub mod random_player;
  pub mod heuristic_player;

  pub mod heuristic;
}

mod test;

fn main () {
  test::test();
  let p1 = strategies::random_player::RandomPlayer::boxed_new();
  let p2 = strategies::heuristic_player::HeuristicPlayer::boxed_new(&strategies::heuristic::heuristic);
  let mut game = game::Game::new(p1, p2);

  let mut win = 0;
  let mut lose = 0;
  let mut draw = 0;

  for i in 0..500 {
    match game.run() {
      0  => {draw += 1;},
      1  => {win  += 1;},
      -1 => {lose += 1;},
      _  => {},
    }
    game.reset();
  }
  print!("w{} d{} l{}", win, draw, lose);
} 