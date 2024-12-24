use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArmourType {
    FullPlate,
    ChainMail,
}

impl Display for ArmourType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmourType::FullPlate => write!(f, "Full Plate"),
            ArmourType::ChainMail => write!(f, "Chainmail"),
        }
    }
}
