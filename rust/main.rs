mod game;
mod game_components;
use game::Board;
use game_components::Action;
use game_components::Player;

fn main () {
  println!("my first rust program");

  let player = Player::X;
  assert!(format!("{}", player) == "Player X");
  assert!(format!("{}", player.opponent()) == "Player O");
  let mut board = Board::new(player);
  assert!(board.all_moves().len() == 36);
  assert!(!board.is_drawed());
  assert!(!board.is_ended());
  assert!(board.is_legal_move(Action::new(0, 0, 5)));
  board.perform_move(Action::new(0, 0, 5));
  assert!(board.is_legal_move(Action::new(0, 2, 1)));
  board.perform_move(Action::new(0, 2, 1));
  assert!(board.is_legal_move(Action::new(0, 2, 1)) == false);
  assert!(board.is_legal_move(Action::new(0, 2, 2)));
  assert!(!board.is_ended());
  board.perform_move(Action::new(0, 1, 4));
  board.perform_move(Action::new(1, 1, 2));
  board.perform_move(Action::new(0, 2, 3));
  assert!(board.is_ended());

  println!("is running without error!!!")
} 