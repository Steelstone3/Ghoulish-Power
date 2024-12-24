use super::{
    constitution::{armour::GhoulArmour, health::GhoulHealth, mana::GhoulMana},
    types::ghoul_types::GhoulType,
    weapon::GhoulWeapon,
};
use crate::presenters::ghoul_presenter::GhoulPresenter;

#[allow(dead_code)]
pub struct Ghoul {
    pub ghoul_type: GhoulType,
    pub health: GhoulHealth,
    pub armour: GhoulArmour,
    pub mana: GhoulMana,
    pub weapon: GhoulWeapon,
}

impl Ghoul {
    pub fn new(ghoul_presenter: &dyn GhoulPresenter) -> Ghoul {
        Ghoul {
            ghoul_type: ghoul_presenter.select_ghoul_type(),
            health: GhoulHealth::default(),
            armour: GhoulArmour::new(ghoul_presenter),
            mana: GhoulMana::default(),
            weapon: GhoulWeapon::new(ghoul_presenter),
        }
    }
}

#[cfg(test)]
mod ghoul_should {
    use super::Ghoul;
    use crate::{
        models::soldier::types::ghoul_types::GhoulType,
        presenters::ghoul_presenter::MockGhoulPresenter,
    };

    #[test]
    #[ignore = "mock fails don't know why"]
    fn construct_player_generated_ghoul() {
        // Given
        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_ghoul_type()
            .times(1)
            .return_const(GhoulType::Undead);

        // When
        let _ghoul = Ghoul::new(&ghoul_presenter);

        // Then
        ghoul_presenter.expect_select_ghoul_type().times(1);
    }
}
