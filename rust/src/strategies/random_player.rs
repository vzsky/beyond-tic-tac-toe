use crate::game_components::Playable;
use crate::game_components::Action;
use crate::game::Board;
use rand::seq::SliceRandom;

// Random Player plays randomly from all possible moves

pub struct RandomPlayer {}
impl RandomPlayer {
  pub fn new () -> RandomPlayer {
    RandomPlayer {}
  }
  pub fn boxed_new () -> Box<RandomPlayer> {
    Box::new(Self::new())
  }
}

impl Playable for RandomPlayer {
  fn get_next_action (&self, board:&Board) -> Action {
    let all_actions = board.all_actions();
    let mut rng = rand::thread_rng();
    *all_actions.choose(&mut rng).unwrap()
  }
}