use crate::BOARD_SIZE;
use crate::STACK_AMOUNT;
use crate::STACK_SIZE;

use crate::game_components::Action;
use crate::game_components::Cell;
use crate::game_components::Player;
use crate::game_components::Playable;
use crate::game_components::Result;


#[derive(Copy, Clone)]
pub struct Board {
  pub now_player : Player,
  pub cells : [[Cell; BOARD_SIZE]; BOARD_SIZE],
  pub stacks : [[usize; STACK_SIZE]; 2],
}

impl Board {

  pub fn new (now_player: Player) -> Board {
    let cells = [[Cell::new(); BOARD_SIZE]; BOARD_SIZE];
    let stacks = [[STACK_AMOUNT; STACK_SIZE]; 2];
    Board {
      now_player,
      cells,
      stacks,
    }
  }

  pub fn is_legal_action (&self, action:Action) -> bool {
    let Action {row, col, size} = action;
    if let Some(playing) = self.cells[row][col].owner {
      if playing == self.now_player {
        return false;
      }
    }
    if self.cells[row][col].size >= size {
      return false;
    }
    self.stacks[self.now_player.number()][size-1] != 0
  }

  pub fn perform_action (&mut self, action:Action) {
    let Action {row, col, size} = action;
    assert!(!self.is_ended());
    assert!(self.is_legal_action(action));
    self.cells[row][col].place(self.now_player, size);
    self.stacks[self.now_player.number()][size-1] -= 1;
    self.now_player = self.now_player.opponent();
  }

  pub fn all_actions (&self) -> Vec<Action> {
    let mut actions = Vec::new();
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        for k in 1..=STACK_SIZE {
          if self.is_legal_action(Action::new(i, j, k)) {
            actions.push(Action::new(i, j, k));
          }
        }
      }
    }
    actions
  }

  pub fn get_winner (&self) -> Option<Player> {
    let mut status:bool;

    for p in [Player::X, Player::O] {
      macro_rules! check {
        ($status:expr, $index:expr) => {
          if !match $index {
            None => false,
            Some(player) if player == p => true,
            Some(_player) => false,
          } { status = false; }
        };
      }

      for i in 0..BOARD_SIZE {
        status = true;
        for j in 0..BOARD_SIZE { check!(status, self.cells[i][j].owner); }
        if status { return Some(p); }
      }

      for j in 0..BOARD_SIZE {
        status = true;
        for i in 0..BOARD_SIZE { check!(status, self.cells[i][j].owner); }
        if status { return Some(p); }
      }
      
      status = true;
      for i in 0..BOARD_SIZE { check!(status, self.cells[i][i].owner); }
      if status { return Some(p); }
      
      status = true;
      for i in 0..BOARD_SIZE { check!(status, self.cells[i][BOARD_SIZE-i-1].owner); }
      if status { return Some(p); }

    }
    None
  }

  pub fn is_drawed (&self) -> bool {
    self.all_actions().len() == 0
  }

  pub fn is_ended (&self) -> bool {
    let winner = self.get_winner();
    if let Some(_player) = winner {
      return true;
    }
    self.is_drawed()
  }

  pub fn ended_value (&self, perspective:Player) -> i32 {
    assert!(self.is_ended());
    if let Some(player) = self.get_winner() {
      if player == perspective { return 1; }
      if player == perspective.opponent() { return -1; }
    }
    assert!(self.is_drawed());
    return 0;
  }

  pub fn print_board (&self) {
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        print!("|{}|", self.cells[i][j]);
      }
      println!();
    }
    println!();
  }
}

pub struct Game {
  pub players: [Box<dyn Playable>; 2], 
  pub board: Board,
  pub first_player: Player
}

impl Game {

  pub fn new (
    p1: Box<dyn Playable>, 
    p2: Box<dyn Playable>,
  ) -> Game {
    Game {
      players: [p1, p2],
      board: Board::new(Player::X),
      first_player: Player::X
    }
  }

  fn reset (&mut self) {
    self.board = Board::new(Player::X);
  }

  fn run (&mut self) -> i32 {
    loop {
      let index = self.board.now_player.number();
      let action = self.players[index].get_next_action(&self.board);
      self.board.perform_action(action);
      
      if self.board.is_ended() {
        return self.board.ended_value(self.first_player);
      }
    }
  }

  pub fn runs (&mut self, times: i32) -> Result {
    let mut result = Result::new();
    for _ in 0..times {
      result.count(self.run());
      self.reset();
    }
    result
  }

  pub fn inverse_runs (&mut self, times: i32) -> Result {
    self.players.reverse();
    let mut result = self.runs(times);
    self.players.reverse();
    result.inverse();
    result
  }

  pub fn full_runs (&mut self, times: i32) -> Result {
    let mut result = Result::new();
    result += self.runs(times);
    result += self.inverse_runs(times);
    result
  }
}