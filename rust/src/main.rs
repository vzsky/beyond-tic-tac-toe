mod game;
mod game_components;
mod strategies {
  pub mod first_action_player;
}

mod test;

fn main () {
  test::test();
  let p1 = strategies::first_action_player::FirstActionPlayer::boxed_new();
  let p2 = strategies::first_action_player::FirstActionPlayer::boxed_new();
  let mut game = game::Game::new(p1, p2);
  println!("{}", game.run());
} 