use std::ops;
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

#[derive(PartialEq)]
pub struct Result {
  pub win:  i32,
  pub draw: i32,
  pub lose: i32,
}

impl Result {
  pub fn new () -> Result {
    Result {
      win:0, draw:0, lose:0
    }
  }
  pub fn count (&mut self, ended_value: i32) {
    match ended_value {
      0  => {self.draw += 1;},
      1  => {self.win  += 1;},
      -1 => {self.lose += 1;},
      _  => {},
    }
  }
  pub fn inverse (&mut self) {
    let temp = self.win;
    self.win = self.lose;
    self.lose = temp;
  }
  pub fn to_number (&self, ) -> i32 {
    let total = self.win + self.draw + self.lose;
    let mul = 1000/total;
    assert!(total*mul == 1000);
    (self.win - self.lose)*mul
  }
}

impl ops::Add for Result {
  type Output = Result;
  fn add(self, other: Self) -> Self {
    Self { win: self.win + other.win, draw: self.draw + other.draw, lose: self.lose + other.lose }
  }
}

impl ops::Mul<i32> for Result {
  type Output = Result;
  fn mul(self, other: i32) -> Self {
    Self { win: other*self.win, draw: other*self.draw, lose: other*self.lose }
  }
}

impl ops::AddAssign for Result {
  fn add_assign(&mut self, other: Self) {
      *self = Self { win: self.win + other.win, draw: self.draw + other.draw, lose: self.lose + other.lose };
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

impl fmt::Display for Result {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "w{} d{} l{}", self.win, self.draw, self.lose)
  }
}