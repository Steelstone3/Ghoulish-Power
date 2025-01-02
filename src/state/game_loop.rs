use crate::presenters::console::Presenter;

use super::{game::GhoulishPower, states::State};
use mockall::automock;

pub struct GameLoop;

impl GameLoop {
    fn new_game(&self, _game: &mut GhoulishPower, presenter: &dyn Presenter) {
        presenter.print("New game");
    }

    fn game_loop(&self, _game: &mut GhoulishPower) {}

    fn game_over(&self, _game: &mut GhoulishPower) {}
}

impl GameLooper for GameLoop {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) {
        match game.state {
            State::NewGame => self.new_game(game, presenter),
            State::GameLoop => self.game_loop(game),
            State::GameOver => self.game_over(game),
        }
    }
}

#[automock]
pub trait GameLooper {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter);
}

#[cfg(test)]
mod game_loop_should {
    use mockall::predicate::eq;

    use crate::{
        models::soldier::{
            constitution::{armour::GhoulArmour, health::GhoulHealth, mana::GhoulMana},
            elements::Element,
            ghoul::Ghoul,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
            weapon::GhoulWeapon,
        },
        presenters::console::MockPresenter,
        state::{game::GhoulishPower, states::State},
    };

    use super::GameLoop;

    #[test]
    fn run_new_game() {
        // Given
        let ghoul = Ghoul {
            ghoul_type: GhoulType::Dark,
            health: GhoulHealth { health: 100 },
            armour: GhoulArmour {
                armour: 100,
                armour_type: ArmourType::ChainMail,
                armour_element: Element::Air,
            },
            mana: GhoulMana { mana: 100 },
            weapon: GhoulWeapon {
                weapon_type: WeaponType::Katana,
                weapon_element: Element::Cosmic,
                damage: 5..10,
            },
        };

        let mut game = GhoulishPower {
            state: State::NewGame,
            player: ghoul,
            enemies: vec![],
        };

        let mut presenter = MockPresenter::new();
        presenter.expect_print().with(eq("New game")).once();

        let game_loop = GameLoop;

        // When
        game_loop.new_game(&mut game, &presenter);
    }

    #[test]
    #[ignore = "not implemented"]
    fn run_game_loop() {}

    #[test]
    #[ignore = "not implemented"]
    fn run_game_over() {}
}
