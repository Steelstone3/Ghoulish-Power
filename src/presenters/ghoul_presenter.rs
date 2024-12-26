use crate::models::soldier::{
    elements::Element,
    types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
};
use inquire::{error::InquireError, Select};

pub struct GhoulConsole;

impl GhoulPresenter for GhoulConsole {
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
            &Element::Fire,
            &Element::Water,
            &Element::Air,
            &Element::Earth,
        ];

        let selection: Result<&Element, InquireError> =
            Select::new(&format!("Select {} element:", message), options).prompt();

        if let Ok(choice) = selection {
            choice.to_owned()
        } else {
            Element::default()
        }
    }
}

#[allow(dead_code)]
pub trait GhoulPresenter {
    fn select_ghoul_type(&self) -> GhoulType;
    fn select_weapon_type(&self) -> WeaponType;
    fn select_armour_type(&self) -> ArmourType;
    fn select_element(&self, message: String) -> Element;
}
