use crate::game_components::Playable;
use crate::game_components::Action;
use crate::game::Board;

struct ActionEval {
  value: f64,
  action: Option<Action>,
}
impl ActionEval {
  fn include (&mut self, eval: &ActionEval) {
    if eval.value > self.value {
      self.value = eval.value;
      self.action = eval.action;
    }
  }
}

pub struct HeuristicPlayer<'a> {
  heuristic: &'a dyn Fn(&Board) -> f64,
}
impl<'a> HeuristicPlayer<'a> {
  pub fn new (heuristic: &'a dyn Fn(&Board) -> f64) -> HeuristicPlayer {
    HeuristicPlayer {
      heuristic
    }
  }
  pub fn boxed_new (heuristic: &'a dyn Fn(&Board) -> f64) -> Box<HeuristicPlayer> {
    Box::new(Self::new(heuristic))
  }

  fn search_action (&self, depth:i32, board:&Board) -> ActionEval {
    if board.is_ended() {
      return ActionEval { value:board.ended_value(board.now_player) as f64, action:None };
    }
    if depth == 0 {
      return ActionEval { value:(self.heuristic)(board), action:None };
    }

    let mut eval = ActionEval {
      value: f64::MIN,
      action: None,
    };

    for action in board.all_actions() {
      let mut new_board = board.clone();
      new_board.perform_action(action);
      let sub_eval = ActionEval {
        value: self.search_action(depth-1, &new_board).value,
        action: Some(action),
      };
      eval.include(&sub_eval);
    }
    eval
  }
}

impl<'a> Playable for HeuristicPlayer<'a> {
  fn get_next_action (&self, board:&Board) -> Action {
    let action = self.search_action(2, board).action.unwrap();
    return action;
  }
}