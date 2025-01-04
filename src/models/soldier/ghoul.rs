use std::default;

use super::{
    constitution::{armour::GhoulArmour, health::GhoulHealth, mana::GhoulMana},
    types::ghoul_types::GhoulType,
    weapon::{GhoulWeapon, Weapon},
};
use crate::presenters::ghoul_presenter::GhoulPresenter;
use rand::random;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Ghoul {
    pub is_dead: bool,
    pub ghoul_type: GhoulType,
    pub health: GhoulHealth,
    pub armour: GhoulArmour,
    pub mana: GhoulMana,
    pub weapon: GhoulWeapon,
}

impl Ghoul {
    pub fn new(ghoul_presenter: &dyn GhoulPresenter) -> Self {
        Self {
            ghoul_type: ghoul_presenter.select_ghoul_type(),
            health: GhoulHealth::default(),
            armour: GhoulArmour::new(ghoul_presenter),
            mana: GhoulMana::default(),
            weapon: GhoulWeapon::new(ghoul_presenter),
            is_dead: Default::default(),
        }
    }

    pub fn new_random() -> Self {
        Self {
            ghoul_type: random(),
            health: GhoulHealth::default(),
            armour: GhoulArmour::new_random(),
            mana: GhoulMana::default(),
            weapon: GhoulWeapon::new_random(),
            is_dead: Default::default(),
        }
    }

    pub fn take_damage(&mut self, weapon: &dyn Weapon) {
        let damage = weapon.attack();

        if self.armour.armour > damage {
            self.armour.armour -= damage
        }
        //  else if self.health.health > damage {
        //     self.health.health -= damage
        // } else {
        //     self.is_dead = true;
        // }
    }
}

#[cfg(test)]
mod ghoul_should {
    use super::Ghoul;
    use crate::{
        models::soldier::{
            constitution::{armour::GhoulArmour, health::GhoulHealth, mana::GhoulMana},
            elements::Element,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
            weapon::GhoulWeapon,
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
    };
    use mockall::predicate::eq;
    use rstest::rstest;

    #[test]
    fn construct_player_generated_ghoul() {
        // Given
        let ghoul_type = GhoulType::Undead;

        let health = GhoulHealth { health: 100 };

        let armour_type = ArmourType::FullPlate;
        let armour_element = Element::Fire;
        let armour = GhoulArmour {
            armour: 100,
            armour_type,
            armour_element,
        };

        let mana = GhoulMana { mana: 100 };

        let weapon_type = WeaponType::Sword;
        let weapon_element = Element::Fire;
        let weapon = GhoulWeapon {
            weapon_type,
            weapon_element,
            damage: 5..10,
        };

        let expected_ghoul = Ghoul {
            ghoul_type,
            health,
            armour,
            mana,
            weapon,
            is_dead: Default::default(),
        };

        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_ghoul_type()
            .once()
            .return_const(ghoul_type);
        ghoul_presenter
            .expect_select_weapon_type()
            .once()
            .return_const(weapon_type);
        ghoul_presenter
            .expect_select_armour_type()
            .once()
            .return_const(armour_type);
        ghoul_presenter
            .expect_select_element()
            .with(eq("armour".to_string()))
            .once()
            .return_const(armour_element);
        ghoul_presenter
            .expect_select_element()
            .with(eq("weapon".to_string()))
            .once()
            .return_const(weapon_element);

        // When
        let ghoul = Ghoul::new(&ghoul_presenter);

        // Then
        assert_eq!(expected_ghoul, ghoul);
    }

    #[rstest]
    #[case(100, 100, 0, false)]
    #[case(90, 100, 10, false)]
    fn take_damage(
        #[case] expected_armour: u32,
        #[case] expected_health: u32,
        #[case] damage: u32,
        #[case] expected_is_dead: bool,
    ) {
        // Given
        let weapon = weapon_test_fixture(damage);
        let mut ghoul = Ghoul {
            is_dead: Default::default(),
            ghoul_type: Default::default(),
            health: Default::default(),
            armour: GhoulArmour {
                armour: 100,
                armour_type: Default::default(),
                armour_element: Default::default(),
            },
            mana: Default::default(),
            weapon: weapon_test_fixture(damage),
        };

        // When
        ghoul.take_damage(&weapon);

        // Then
        assert_eq!(expected_armour, ghoul.armour.armour);
        assert_eq!(expected_health, ghoul.health.health);
        assert_eq!(expected_is_dead, ghoul.is_dead);
    }

    fn weapon_test_fixture(damage: u32) -> GhoulWeapon {
        GhoulWeapon {
            weapon_type: Default::default(),
            weapon_element: Default::default(),
            damage: damage..damage,
        }
    }
}
