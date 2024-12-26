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
    use mockall::predicate::eq;

    use super::Ghoul;
    use crate::{
        models::soldier::{
            elements::Element,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
    };

    #[test]
    fn construct_player_generated_ghoul() {
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
        let ghoul = Ghoul::new(&ghoul_presenter);

        // Then
        assert_eq!(100, ghoul.health.health);
        assert_eq!(100, ghoul.armour.armour);
        assert_eq!(100, ghoul.mana.mana);
    }
}
