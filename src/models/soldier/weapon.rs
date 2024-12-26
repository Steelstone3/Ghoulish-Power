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
    use crate::{
        models::soldier::{
            elements::Element,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
            weapon::GhoulWeapon,
        },
        presenters::ghoul_presenter::{GhoulPresenter, MockGhoulPresenter},
    };

    #[test]
    #[ignore = "not implemented"]
    fn construct_player_generated_weapon() {
        // Given
        let ghoul_presenter = MockGhoulPresenter::new();
        // {
        //     select_ghoul_type: Mock::default(),
        //     select_weapon_type: Mock::default(),
        //     select_armour_type: Mock::default(),
        //     select_element: Mock::default(),
        // };

        // When
        let ghoul_weapon = GhoulWeapon::new(&ghoul_presenter);

        // Then
        // assert_eq!(1, ghoul_presenter.select_weapon_type.num_calls());
        // assert_eq!(1, ghoul_presenter.select_element.num_calls());
        // assert_eq!(0, ghoul_presenter.select_ghoul_type.num_calls());
        // assert_eq!(0, ghoul_presenter.select_armour_type.num_calls());
        assert_eq!(5..10, ghoul_weapon.damage);
    }
}
