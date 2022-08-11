use std::fmt;
use crate::game::Board;

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
  X, 
  O,
}

impl Player {
  pub fn opponent (&self) -> Player {
    match *self {
      Player::X => Player::O,
      Player::O => Player::X,
    }
  }
  pub fn number (&self) -> usize {
    match *self {
      Player::X => 0,
      Player::O => 1,
    }
  }
}

pub trait Playable {
  fn get_next_action (&self, board:&Board) -> Action;
}

#[derive(Copy, Clone)]
pub struct Action {
  pub row: usize,
  pub col: usize,
  pub size: usize,
}

impl Action {
  pub fn new (row:usize, col:usize, size:usize) -> Action {
    Action {
      row, col, size
    }
  }
}

#[derive(Copy, Clone)]
pub struct Cell {
  pub owner: Option<Player>,
  pub size: usize,
}

impl Cell {
  pub fn new () -> Cell {
    Cell {
      owner: None,
      size: 0
    }
  }
  pub fn place (&mut self, player: Player, size: usize) {
    self.owner = Some(player);
    self.size = size;
  }
}

impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Player::X => write!(f, "X"),
      Player::O => write!(f, "O"),
    }
  }
}

impl fmt::Display for Action {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(r{}, c{}, s{}) ", self.row, self.col, self.size)
  }
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(player) = self.owner {
      return write!(f, "{}{}", player, self.size);
    }
    write!(f, "__")
  }
}