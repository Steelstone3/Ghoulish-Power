use super::{game_loop::GameLooper, states::State};
use crate::{models::soldier::ghoul::Ghoul, presenters::ghoul_presenter::GhoulPresenter};

#[allow(dead_code)]
pub struct GhoulishPower {
    pub state: State,
    pub player: Ghoul,
}

impl GhoulishPower {
    pub fn new(ghoul_presenter: &dyn GhoulPresenter) -> Self {
        Self {
            state: Default::default(),
            player: Ghoul::new(ghoul_presenter),
        }
    }

    #[allow(dead_code)]
    pub fn game_loop(&mut self, _game_loop: &dyn GameLooper) {}
}

#[cfg(test)]
mod game_should {
    #[test]
    #[ignore = "work out why mocks aren't working then create a stub for ghoul presenter for Ghoul::new(ghoul_presenter)"]
    fn construct() {
        // Given

        // When

        // Then
    }
}
