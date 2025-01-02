use presenters::console::Console;
use state::{
    game::{Game, GhoulishPower},
    game_loop::GameLoop,
};

mod controllers;
mod models;
mod presenters;
mod state;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
fn main() {
    let mut ghoulish_power = GhoulishPower::new(&Console);
    let game_loop = GameLoop;

    ghoulish_power.game_loop(&game_loop);
}
