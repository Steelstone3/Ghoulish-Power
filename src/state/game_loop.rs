use super::game::GhoulishPower;
use mockall::automock;

pub struct GameLoop;

impl GameLooper for GameLoop {
    fn run(&self, _game: &mut GhoulishPower) {}
}

#[automock]
pub trait GameLooper {
    fn run(&self, game: &mut GhoulishPower);
}

#[cfg(test)]
mod game_loop_should {
    #[test]
    #[ignore = "not implemented"]
    fn run() {}
}
