use super::{game::GhoulishPower, states::State};
use mockall::automock;

pub struct GameLoop;

impl GameLoop {
    fn new_game(&self, _game: &mut GhoulishPower) {}

    fn game_loop(&self, _game: &mut GhoulishPower) {}

    fn game_over(&self, _game: &mut GhoulishPower) {}
}

impl GameLooper for GameLoop {
    fn run(&self, game: &mut GhoulishPower) {
        match game.state {
            State::NewGame => self.new_game(game),
            State::GameLoop => self.game_loop(game),
            State::GameOver => self.game_over(game),
        }
    }
}

#[automock]
pub trait GameLooper {
    fn run(&self, game: &mut GhoulishPower);
}

#[cfg(test)]
mod game_loop_should {
    #[test]
    #[ignore = "not implemented"]
    fn run_new_game() {}

    #[test]
    #[ignore = "not implemented"]
    fn run_game_loop() {}

    #[test]
    #[ignore = "not implemented"]
    fn run_game_over() {}
}
