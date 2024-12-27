use crate::presenters::ghoul_presenter::GhoulPresenter;

use super::{elements::Element, types::weapon_types::WeaponType};
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct GhoulWeapon {
    pub weapon_type: WeaponType,
    pub weapon_element: Element,
    pub damage: Range<u32>,
}

impl GhoulWeapon {
    pub fn new(ghoul_presenter: &dyn GhoulPresenter) -> Self {
        Self {
            weapon_type: ghoul_presenter.select_weapon_type(),
            weapon_element: ghoul_presenter.select_element("weapon".to_string()),
            damage: 5..10,
        }
    }
}

#[allow(dead_code)]
pub trait Weapon {}

#[cfg(test)]
mod weapon_should {
    use crate::{
        models::soldier::{
            elements::Element, types::weapon_types::WeaponType, weapon::GhoulWeapon,
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
    };
    use mockall::predicate::eq;

    #[test]
    fn construct_player_generated_weapon() {
        // Given
        let weapon_type = WeaponType::Sword;
        let weapon_element = Element::Fire;
        let expected_ghoul_weapon = GhoulWeapon {
            weapon_type,
            weapon_element,
            damage: 5..10,
        };

        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_weapon_type()
            .once()
            .return_const(weapon_type);
        ghoul_presenter
            .expect_select_element()
            .with(eq("weapon".to_string()))
            .once()
            .return_const(weapon_element);

        // When
        let ghoul_weapon = GhoulWeapon::new(&ghoul_presenter);

        // Then
        assert_eq!(expected_ghoul_weapon, ghoul_weapon);
    }
}
