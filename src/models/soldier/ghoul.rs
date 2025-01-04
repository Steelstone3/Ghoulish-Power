use super::{
    constitution::{
        armour::GhoulArmour,
        health::GhoulHealth,
        mana::GhoulMana,
    },
    types::ghoul_types::GhoulType,
    weapon::{GhoulWeapon, Weapon},
};
use crate::presenters::ghoul_presenter::GhoulPresenter;
use rand::random;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Ghoul {
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
        }
    }

    pub fn new_random() -> Self {
        Self {
            ghoul_type: random(),
            health: GhoulHealth::default(),
            armour: GhoulArmour::new_random(),
            mana: GhoulMana::default(),
            weapon: GhoulWeapon::new_random(),
        }
    }

    pub fn take_damage(&mut self, weapon: &dyn Weapon) {
        todo!()
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
}
