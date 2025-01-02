use super::console::Console;
use crate::models::soldier::{
    elements::Element,
    types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
};
use inquire::{error::InquireError, Select};
use mockall::automock;

impl GhoulPresenter for Console {
    fn select_ghoul_type(&self) -> GhoulType {
        let options: Vec<&GhoulType> = vec![
            &GhoulType::Undead,
            &GhoulType::Light,
            &GhoulType::Dark,
            &GhoulType::Death,
            &GhoulType::Soulless,
        ];

        let selection: Result<&GhoulType, InquireError> =
            Select::new("Select ghoul type:", options).prompt();

        if let Ok(choice) = selection {
            choice.to_owned()
        } else {
            GhoulType::default()
        }
    }

    fn select_weapon_type(&self) -> WeaponType {
        let options: Vec<&WeaponType> = vec![&WeaponType::Sword, &WeaponType::Katana];

        let selection: Result<&WeaponType, InquireError> =
            Select::new("Select weapon type:", options).prompt();

        if let Ok(choice) = selection {
            choice.to_owned()
        } else {
            WeaponType::default()
        }
    }

    fn select_armour_type(&self) -> ArmourType {
        let options: Vec<&ArmourType> = vec![&ArmourType::FullPlate, &ArmourType::ChainMail];

        let selection: Result<&ArmourType, InquireError> =
            Select::new("Select armour type:", options).prompt();

        if let Ok(choice) = selection {
            choice.to_owned()
        } else {
            ArmourType::default()
        }
    }

    fn select_element(&self, message: String) -> Element {
        let options: Vec<&Element> = vec![
            &Element::Air,
            &Element::Cosmic,
            &Element::Earth,
            &Element::Fire,
            &Element::Force,
            &Element::Frost,
            &Element::Water,
        ];

        let selection: Result<&Element, InquireError> =
            Select::new(&format!("Select {message} element:"), options).prompt();

        if let Ok(choice) = selection {
            choice.to_owned()
        } else {
            Element::default()
        }
    }
}

#[automock]
pub trait GhoulPresenter {
    fn select_ghoul_type(&self) -> GhoulType;
    fn select_weapon_type(&self) -> WeaponType;
    fn select_armour_type(&self) -> ArmourType;
    fn select_element(&self, message: String) -> Element;
}
