use mockall::automock;

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
}

impl Game for GhoulishPower {
    #[allow(dead_code)]
    fn game_loop(&mut self, _game_loop: &dyn GameLooper) {}
}

#[automock]
pub trait Game {
    fn game_loop(&mut self, _game_loop: &dyn GameLooper);
}

#[cfg(test)]
mod game_should {
    use crate::{
        models::soldier::{
            elements::Element,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
        state::{game::GhoulishPower, states::State},
    };
    use mockall::predicate::eq;

    #[test]
    fn construct() {
        // Given
        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_ghoul_type()
            .once()
            .return_const(GhoulType::Undead);
        ghoul_presenter
            .expect_select_weapon_type()
            .once()
            .return_const(WeaponType::Sword);
        ghoul_presenter
            .expect_select_armour_type()
            .once()
            .return_const(ArmourType::FullPlate);
        ghoul_presenter
            .expect_select_element()
            .with(eq("armour".to_string()))
            .once()
            .return_const(Element::Fire);
        ghoul_presenter
            .expect_select_element()
            .with(eq("weapon".to_string()))
            .once()
            .return_const(Element::Fire);

        // When
        let ghoulish_power = GhoulishPower::new(&ghoul_presenter);

        // Then
        assert_eq!(100, ghoulish_power.player.health.health);
        assert_eq!(100, ghoulish_power.player.armour.armour);
        assert_eq!(100, ghoulish_power.player.mana.mana);
        assert_eq!(State::NewGame, ghoulish_power.state);
    }
}
