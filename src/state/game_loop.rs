use crate::presenters::console::Presenter;

use super::{game::GhoulishPower, states::State};
use mockall::automock;

pub struct GameLoop;

impl GameLoop {
    fn new_game(&self, _game: &mut GhoulishPower, presenter: &dyn Presenter) {
        presenter.print(message);
    }

    fn game_loop(&self, _game: &mut GhoulishPower) {}

    fn game_over(&self, _game: &mut GhoulishPower) {}
}

impl GameLooper for GameLoop {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) {
        match game.state {
            State::NewGame => self.new_game(game, presenter),
            State::GameLoop => self.game_loop(game),
            State::GameOver => self.game_over(game),
        }
    }
}

#[automock]
pub trait GameLooper {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter);
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
