use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(Clone, Copy, Debug, PartialEq, Default, RandGen)]
pub enum WeaponType {
    Katana,
    #[default]
    Sword,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::Katana => write!(f, "Katana"),
            WeaponType::Sword => write!(f, "Sword"),
        }
    }
}

#[cfg(test)]
mod weapon_type_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(WeaponType::Sword, "Sword")]
    #[case(WeaponType::Katana, "Katana")]
    fn display(#[case] weapon_type: WeaponType, #[case] expected_display: &str) {
        // Then
        assert_eq!(expected_display, weapon_type.to_string())
    }
}
