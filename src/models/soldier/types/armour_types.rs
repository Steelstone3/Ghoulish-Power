use std::fmt::Display;

use rand_derive2::RandGen;

#[derive(Clone, Copy, Debug, PartialEq, Default, RandGen)]
pub enum ArmourType {
    ChainMail,
    #[default]
    FullPlate,
}

impl Display for ArmourType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmourType::ChainMail => write!(f, "Chainmail"),
            ArmourType::FullPlate => write!(f, "Full Plate"),
        }
    }
}

#[cfg(test)]
mod armour_type_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(ArmourType::ChainMail, "Chainmail")]
    #[case(ArmourType::FullPlate, "Full Plate")]
    fn display(#[case] armour_type: ArmourType, #[case] expected_display: &str) {
        // Then
        assert_eq!(expected_display, armour_type.to_string())
    }
}
