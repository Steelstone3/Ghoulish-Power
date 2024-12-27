use crate::{
    models::soldier::{elements::Element, types::armour_types::ArmourType},
    presenters::ghoul_presenter::GhoulPresenter,
};

#[derive(Debug, PartialEq)]
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
            constitution::armour::GhoulArmour, elements::Element, types::armour_types::ArmourType,
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
    };
    use mockall::predicate::eq;

    #[test]
    fn construct_player_generated_armour() {
        // Given
        let armour_type = ArmourType::ChainMail;
        let armour_element = Element::Fire;
        let expected_ghoul_armour = GhoulArmour {
            armour: 100,
            armour_type,
            armour_element,
        };

        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_armour_type()
            .once()
            .return_const(armour_type);
        ghoul_presenter
            .expect_select_element()
            .with(eq("armour".to_string()))
            .once()
            .return_const(armour_element);

        // When
        let ghoul_armour = GhoulArmour::new(&ghoul_presenter);

        // Then
        assert_eq!(expected_ghoul_armour, ghoul_armour);
    }
}
