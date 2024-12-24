#[allow(dead_code)]
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
