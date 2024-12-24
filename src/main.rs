use presenters::ghoul_presenter::GhoulConsole;
use state::game::GhoulishPower;

mod controllers;
mod models;
mod presenters;
mod state;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
fn main() {
    let _ghoulish_power = GhoulishPower::new(&GhoulConsole);
}
