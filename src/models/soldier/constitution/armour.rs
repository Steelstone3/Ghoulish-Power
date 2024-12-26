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
    use mockall::predicate::eq;

    use crate::{
        models::soldier::{
            constitution::armour::GhoulArmour, elements::Element, types::armour_types::ArmourType,
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
    };

    #[test]
    fn construct_player_generated_armour() {
        // Given
        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_armour_type()
            .once()
            .return_const(ArmourType::ChainMail);
        ghoul_presenter
            .expect_select_element()
            .with(eq("armour".to_string()))
            .once()
            .return_const(Element::Fire);

        // When
        let ghoul_armour = GhoulArmour::new(&ghoul_presenter);

        // Then
        assert_eq!(100, ghoul_armour.armour);
    }
}
