#![allow(dead_code)]
#![allow(unused_imports)]

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

#[allow(unused_variables)]
fn main () {
  test::test();
  let ran1 = strategies::random_player::RandomPlayer::boxed_new();
  let ran2 = strategies::random_player::RandomPlayer::boxed_new();
  let heu1 = strategies::heuristic_player::HeuristicPlayer::boxed_new(&strategies::heuristic::heuristic1);
  let heu2 = strategies::heuristic_player::HeuristicPlayer::boxed_new(&strategies::heuristic::heuristic2);
  
  // we really only cares about p1 strategies since p2 can force a draw
  let mut game = game::Game::new(heu1, ran1);
  let game_result = game.runs(500);
  println!("{}", game_result);

  let mut game = game::Game::new(heu2, ran2);
  let game_result = game.runs(500);
  println!("{}", game_result);
  
  // random vs hue1 w84 d285 l131
  // - python got [244, 162, 94]
  // hue1 vs random w172 d208 l120
  // - python got [392, 93, 15]
  // random vs random w245 d93 l162
  // - python got [185, 232, 83]
} 