use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum WeaponType {
    #[default]
    Sword,
    Katana,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::Sword => write!(f, "Sword"),
            WeaponType::Katana => write!(f, "Katana"),
        }
    }
}
