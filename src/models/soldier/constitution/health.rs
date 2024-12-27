#[derive(Debug, PartialEq)]
pub struct GhoulHealth {
    pub health: u32,
}

impl Default for GhoulHealth {
    fn default() -> Self {
        Self { health: 100 }
    }
}

#[allow(dead_code)]
pub trait Health {}

#[cfg(test)]
mod health_should {
    use super::*;

    #[test]
    fn construct() {
        // Given
        let expected_health = 100;
        let ghoul_health = GhoulHealth::default();

        // Then
        assert_eq!(expected_health, ghoul_health.health);
    }
}
