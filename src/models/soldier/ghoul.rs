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
        models::soldier::{
            elements::Element,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
        },
        presenters::ghoul_presenter::GhoulPresenter,
    };
    use pseudo::Mock;

    #[test]
    fn construct_player_generated_ghoul() {
        // Given
        let ghoul_presenter = MockGhoulPresenter {
            select_ghoul_type: Mock::default(),
            select_weapon_type: Mock::default(),
            select_armour_type: Mock::default(),
            select_element: Mock::default(),
        };

        // When
        let ghoul = Ghoul::new(&ghoul_presenter);

        // Then
        assert_eq!(1, ghoul_presenter.select_ghoul_type.num_calls());
        assert_eq!(1, ghoul_presenter.select_weapon_type.num_calls());
        assert_eq!(1, ghoul_presenter.select_armour_type.num_calls());
        assert_eq!(2, ghoul_presenter.select_element.num_calls());
        assert_eq!(100, ghoul.health.health);
        assert_eq!(100, ghoul.armour.armour);
        assert_eq!(100, ghoul.mana.mana);
    }

    pub struct MockGhoulPresenter {
        pub select_ghoul_type: Mock<(), GhoulType>,
        pub select_weapon_type: Mock<(), WeaponType>,
        pub select_armour_type: Mock<(), ArmourType>,
        pub select_element: Mock<(), Element>,
    }

    impl GhoulPresenter for MockGhoulPresenter {
        fn select_ghoul_type(&self) -> GhoulType {
            self.select_ghoul_type.call(());
            GhoulType::default()
        }

        fn select_weapon_type(&self) -> WeaponType {
            self.select_weapon_type.call(());
            WeaponType::default()
        }

        fn select_armour_type(&self) -> ArmourType {
            self.select_armour_type.call(());
            ArmourType::default()
        }

        fn select_element(&self, _: String) -> Element {
            self.select_element.call(());
            Element::default()
        }
    }
}
