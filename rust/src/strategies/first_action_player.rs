use crate::game_components::Playable;
use crate::game_components::Action;
use crate::game::Board;

// First Action Player is the most stupid player out there.
// It plays first lexicographical action.
// It's dumb, but deterministic.

pub struct FirstActionPlayer {}
impl FirstActionPlayer {
  pub fn new () -> FirstActionPlayer {
    FirstActionPlayer {}
  }
  pub fn boxed_new () -> Box<FirstActionPlayer> {
    Box::new(Self::new())
  }
}

impl Playable for FirstActionPlayer {
  fn get_next_move (&self, board:&Board) -> Action {
    let all_moves = board.all_moves();
    all_moves[0]
  }
}