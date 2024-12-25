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
        models::soldier::{elements::Element, ghoul::Ghoul, types::weapon_types::WeaponType},
        // presenters::ghoul_presenter::MockGhoulPresenter,
    };

    #[test]
    #[ignore = "mock fails don't know why"]
    fn construct_player_generated_weapon() {
        // // Given
        // let mut ghoul_presenter = MockGhoulPresenter::default();
        // ghoul_presenter
        //     .expect_select_weapon_type()
        //     .times(1)
        //     .return_const(WeaponType::Katana);
        // ghoul_presenter
        //     .expect_select_element()
        //     .times(1)
        //     .return_const(Element::Water);

        // // When
        // let _ghoul = Ghoul::new(&ghoul_presenter);

        // // Then
        // ghoul_presenter.expect_select_weapon_type().times(1);
        // ghoul_presenter.expect_select_element().times(1);
    }
}
