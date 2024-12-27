#[derive(Debug, PartialEq)]
pub struct GhoulMana {
    pub mana: u32,
}

impl Default for GhoulMana {
    fn default() -> Self {
        Self { mana: 100 }
    }
}

#[allow(dead_code)]
pub trait Mana {}

#[cfg(test)]
mod health_should {
    use super::*;

    #[test]
    fn construct() {
        // Given
        let expected_mana = 100;
        let ghoul_mana = GhoulMana::default();

        // Then
        assert_eq!(expected_mana, ghoul_mana.mana);
    }
}
