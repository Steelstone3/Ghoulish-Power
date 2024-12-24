#[allow(dead_code)]
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
