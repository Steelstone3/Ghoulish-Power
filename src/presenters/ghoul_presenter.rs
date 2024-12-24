use crate::models::soldier::{
    elements::Element,
    types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
};
use inquire::{error::InquireError, Select};
use mockall::automock;

pub struct GhoulConsole;

impl GhoulPresenter for GhoulConsole {
    fn select_ghoul_type(&self) -> GhoulType {
        let options: Vec<&GhoulType> = vec![&GhoulType::Undead];

        let ans: Result<&GhoulType, InquireError> =
            Select::new("Select ghoul type:", options).prompt();

        if let Ok(choice) = ans {
            choice.to_owned()
        } else {
            GhoulType::Undead
        }
    }

    fn select_weapon_type(&self) -> WeaponType {
        WeaponType::Katana
    }

    fn select_armour_type(&self) -> ArmourType {
        ArmourType::ChainMail
    }

    fn select_element(&self) -> Element {
        let options: Vec<&Element> = vec![
            &Element::Fire,
            &Element::Water,
            &Element::Air,
            &Element::Earth,
        ];

        let ans: Result<&Element, InquireError> = Select::new("Select element:", options).prompt();

        if let Ok(choice) = ans {
            choice.to_owned()
        } else {
            Element::Air
        }
    }
}

#[allow(dead_code)]
#[automock]
pub trait GhoulPresenter {
    fn select_ghoul_type(&self) -> GhoulType;
    fn select_weapon_type(&self) -> WeaponType;
    fn select_armour_type(&self) -> ArmourType;
    fn select_element(&self) -> Element;
}
