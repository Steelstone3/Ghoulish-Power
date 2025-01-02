use crate::presenters::console::Presenter;

use super::{game::GhoulishPower, states::State};
use mockall::automock;

pub struct GameLoop;

impl GameLoop {
    fn new_game(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) {
        presenter.print("New game");
        game.state = State::GameLoop;
    }

    fn game_loop(&self, game: &mut GhoulishPower) {
        game.state = State::GameOver;
    }

    fn game_over(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) {
        presenter.print("Game over");
        game.state = State::NewGame;
    }
}

impl GameLooper for GameLoop {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) {
        match game.state {
            State::NewGame => self.new_game(game, presenter),
            State::GameLoop => self.game_loop(game),
            State::GameOver => self.game_over(game, presenter),
        }
    }
}

#[automock]
pub trait GameLooper {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter);
}

#[cfg(test)]
mod game_loop_should {
    use super::GameLoop;
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
    use mockall::predicate::eq;

    #[test]
    fn run_new_game() {
        // Given
        let ghoul = test_fixture_ghoul();

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

        // Then
        assert_eq!(State::GameLoop, game.state)
    }

    #[test]
    fn run_game_loop() {
        // Given
        let ghoul = test_fixture_ghoul();

        let mut game = GhoulishPower {
            state: State::NewGame,
            player: ghoul,
            enemies: vec![],
        };

        // let mut presenter = MockPresenter::new();
        // presenter.expect_print().with(eq("New game")).once();

        let game_loop = GameLoop;

        // When
        game_loop.game_loop(&mut game);

        // Then
        assert_eq!(State::GameOver, game.state)
    }

    #[test]
    fn run_game_over() {
        // Given
        let ghoul = test_fixture_ghoul();

        let mut game = GhoulishPower {
            state: State::NewGame,
            player: ghoul,
            enemies: vec![],
        };

        let mut presenter = MockPresenter::new();
        presenter.expect_print().with(eq("Game over")).once();

        let game_loop = GameLoop;

        // When
        game_loop.game_over(&mut game, &presenter);

        // Then
        assert_eq!(State::NewGame, game.state)
    }

    fn test_fixture_ghoul() -> Ghoul {
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
        ghoul
    }
}
