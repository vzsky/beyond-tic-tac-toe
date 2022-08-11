use crate::game::Board;
use crate::game::Game;
use crate::game_components::Action;
use crate::game_components::Player;

use crate::strategies::first_action_player::FirstActionPlayer;

pub fn test () {
  println!("my first rust program");

  let player = Player::X;
  assert!(format!("{}", player) == "X");
  assert!(format!("{}", player.opponent()) == "O");
  let mut board = Board::new(player);
  assert!(!board.is_drawed());
  assert!(!board.is_ended());
  assert!(board.is_legal_action(Action::new(0, 0, 5)));
  board.perform_action(Action::new(0, 0, 5));
  assert!(board.is_legal_action(Action::new(0, 2, 1)));
  board.perform_action(Action::new(0, 2, 1));
  assert!(board.is_legal_action(Action::new(0, 2, 1)) == false);
  assert!(board.is_legal_action(Action::new(0, 2, 2)));
  assert!(!board.is_ended());
  board.perform_action(Action::new(0, 1, 4));
  board.perform_action(Action::new(1, 1, 2));
  board.perform_action(Action::new(0, 2, 3));
  assert!(board.is_ended());

  let p1 = FirstActionPlayer::new();
  let p2 = FirstActionPlayer::new();
  let mut game: Game = Game::new(Box::new(p1), Box::new(p2));
  let result = game.run();
  assert!(result == 0); // the game is drawed

  println!("is running without error!!!")
}