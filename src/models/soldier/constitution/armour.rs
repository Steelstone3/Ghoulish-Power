use crate::{
    models::soldier::{elements::Element, types::armour_types::ArmourType},
    presenters::ghoul_presenter::GhoulPresenter,
};

#[allow(dead_code)]
pub struct GhoulArmour {
    pub armour: u32,
    pub armour_type: ArmourType,
    pub armour_element: Element,
}

impl GhoulArmour {
    pub fn new(ghoul_presenter: &dyn GhoulPresenter) -> Self {
        Self {
            armour: 100,
            armour_type: ghoul_presenter.select_armour_type(),
            armour_element: ghoul_presenter.select_element("armour".to_string()),
        }
    }
}

#[allow(dead_code)]
pub trait Armour {}

#[cfg(test)]
mod armour_should {
    use crate::{
        models::soldier::{
            constitution::armour::GhoulArmour,
            elements::Element,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
        },
        presenters::ghoul_presenter::{GhoulPresenter, MockGhoulPresenter},
    };

    #[test]
    #[ignore = "not implemented"]
    fn construct_player_generated_armour() {
        // Given
        let ghoul_presenter = MockGhoulPresenter::new();
        // {
        //     select_ghoul_type: Mock::default(),
        //     select_weapon_type: Mock::default(),
        //     select_armour_type: Mock::default(),
        //     select_element: Mock::default(),
        // };

        // When
        let ghoul_armour = GhoulArmour::new(&ghoul_presenter);

        // Then
        // assert_eq!(1, ghoul_presenter.select_armour_type.num_calls());
        // assert_eq!(1, ghoul_presenter.select_element.num_calls());
        // assert_eq!(0, ghoul_presenter.select_ghoul_type.num_calls());
        // assert_eq!(0, ghoul_presenter.select_weapon_type.num_calls());
        assert_eq!(100, ghoul_armour.armour);
    }
}
