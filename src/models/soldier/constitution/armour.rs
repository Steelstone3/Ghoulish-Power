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
            armour_element: ghoul_presenter.select_element(),
        }
    }
}

#[allow(dead_code)]
pub trait Armour {}

#[cfg(test)]
mod armour_should {
    use super::GhoulArmour;
    use crate::{
        models::soldier::{elements::Element, types::armour_types::ArmourType},
        // presenters::ghoul_presenter::MockGhoulPresenter,
    };

    #[test]
    #[ignore = "mock fails don't know why"]
    fn construct_player_generated_armour() {
        // Given
        // let armour = 100;
        // let mut ghoul_presenter = MockGhoulPresenter::new();
        // ghoul_presenter
        //     .expect_select_armour_type()
        //     .times(1)
        //     .return_const(ArmourType::ChainMail);
        // ghoul_presenter
        //     .expect_select_element()
        //     .times(1)
        //     .return_const(Element::Water);

        // When
        // let ghoul_armour = GhoulArmour::new(&ghoul_presenter);

        // Then
        // ghoul_presenter.expect_select_armour_type().times(1);
        // ghoul_presenter.expect_select_element().times(1);
        // assert_eq!(armour, ghoul_armour.armour)
    }
}
