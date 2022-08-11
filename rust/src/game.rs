const BOARD_SIZE:usize = 3;
const STACK_AMOUNT:usize = 1;
const STACK_SIZE:usize = 5;

use crate::game_components::Action;
use crate::game_components::Cell;
use crate::game_components::Player;
use crate::game_components::Playable;

pub struct Board {
  now_player : Player,
  cells : [[Cell; BOARD_SIZE]; BOARD_SIZE],
  stacks : [[usize; STACK_SIZE]; 2],
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

  pub fn is_legal_move (&self, action:Action) -> bool {
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

  pub fn perform_move (&mut self, action:Action) {
    let Action {row, col, size} = action;
    assert!(!self.is_ended());
    assert!(self.is_legal_move(action));
    self.cells[row][col].place(self.now_player, size);
    self.stacks[self.now_player.number()][size-1] -= 1;
    self.now_player = self.now_player.opponent();
  }

  pub fn all_moves (&self) -> Vec<Action> {
    let mut moves = Vec::new();
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        for k in 1..STACK_SIZE {
          if self.is_legal_move(Action::new(i, j, k)) {
            moves.push(Action::new(i, j, k));
          }
        }
      }
    }
    moves
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
    self.all_moves().len() == 0
  }

  pub fn is_ended (&self) -> bool {
    let winner = self.get_winner();
    if let Some(_player) = winner {
      return true;
    }
    self.is_drawed()
  }

}

pub struct Game {
  pub players: [Box<dyn Playable>; 2], 
  pub board: Board,
}

impl Game {

  pub fn new (
    p1: Box<dyn Playable>, 
    p2: Box<dyn Playable>,
  ) -> Game {
    Game {
      players: [p1, p2],
      board: Board::new(Player::X),
    }
  }

  pub fn run (&mut self) -> i32 {
    loop {
      let index = self.board.now_player.number();
      let action = self.players[index].get_next_move(&self.board);
      self.board.perform_move(action);
      
      if self.board.is_ended() {
        if let Some(player) = self.board.get_winner() {
          if player == Player::X { return 1; }
          if player == Player::O { return -1; }
        }
        assert!(self.board.is_drawed());
        return 0;
      }
    }
  }

}