use crate::presenters::ghoul_presenter::GhoulPresenter;

use super::{elements::Element, types::weapon_types::WeaponType};
use std::ops::Range;

#[allow(dead_code)]
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
    use mockall::predicate::eq;

    use crate::{
        models::soldier::{
            elements::Element, types::weapon_types::WeaponType, weapon::GhoulWeapon,
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
    };

    #[test]
    fn construct_player_generated_weapon() {
        // Given
        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_weapon_type()
            .once()
            .return_const(WeaponType::Sword);
        ghoul_presenter
            .expect_select_element()
            .with(eq("weapon".to_string()))
            .once()
            .return_const(Element::Water);

        // When
        let ghoul_weapon = GhoulWeapon::new(&ghoul_presenter);

        // Then
        assert_eq!(5..10, ghoul_weapon.damage);
    }
}
