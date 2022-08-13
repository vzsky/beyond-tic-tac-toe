use crate::BOARD_SIZE;
use crate::STACK_SIZE;

use crate::game_components::Playable;
use crate::game_components::Action;
use crate::game::Board;

pub struct P2DrawPlayer {}
impl P2DrawPlayer {
  pub fn new () -> P2DrawPlayer {
    P2DrawPlayer {}
  }
  pub fn boxed_new () -> Box<P2DrawPlayer> {
    Box::new(Self::new())
  }
}

impl Playable for P2DrawPlayer {
  fn get_next_action (&self, board:&Board) -> Action {
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        if let Some(player) = board.cells[i][j].owner {
          if player == board.now_player.opponent() {
            for k in 1..(STACK_SIZE+1) {
              if board.is_legal_action(Action::new(i, j, k)) {
                return Action::new(i, j, k);
              }
            }
          }
        }
      }
    }
    // if this player cannot replace existing tile for any reason, 
    // this should happens only once in a game
    board.all_actions()[0]
  }
}