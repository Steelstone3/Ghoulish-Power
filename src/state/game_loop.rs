use crate::{
    controllers::random_controller::{RandomController, RandomGenerator},
    models::soldier::ghoul::Ghoul,
    presenters::console::Presenter,
};

use super::{game::GhoulishPower, states::GameState};
use mockall::automock;

pub struct GameLoop;

impl GameLoop {
    fn new_game(&self, presenter: &dyn Presenter) -> GameState {
        presenter.print("New game");
        GameState::GameLoop
    }

    fn game_loop(&self, game: &mut GhoulishPower) -> GameState {
        let ghouls = GameLoop::create_enemies();

        game.enemies = ghouls;

        // while !game.enemies.is_empty() {}

        GameState::GameOver
    }

    #[allow(dead_code)]
    fn create_enemies() -> Vec<Ghoul> {
        let mut ghouls: Vec<Ghoul> = vec![];

        for _ in 1..RandomController::random_value_i32(RandomController::generate_seed(), 1..5) {
            ghouls.push(Ghoul::new_random());
        }

        ghouls
    }

    #[allow(dead_code)]
    fn turn() {
        todo!()
    }

    fn game_over(&self, presenter: &dyn Presenter) -> GameState {
        presenter.print("Game over");
        GameState::NewGame
    }
}

impl GameLooper for GameLoop {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) -> GameState {
        match game.game_state {
            GameState::NewGame => self.new_game(presenter),
            GameState::GameLoop => self.game_loop(game),
            GameState::GameOver => self.game_over(presenter),
        }
    }
}

#[automock]
pub trait GameLooper {
    fn run(&self, game: &mut GhoulishPower, presenter: &dyn Presenter) -> GameState;
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
        state::{
            game::GhoulishPower,
            states::{GameState, State},
        },
    };
    use mockall::predicate::eq;

    #[test]
    fn run_new_game() {
        // Given
        let mut presenter = MockPresenter::new();
        presenter
            .expect_print()
            .with(eq("New game"))
            .once()
            .returning(|_| ());

        let game_loop = GameLoop;

        // When
        let state = game_loop.new_game(&presenter);

        // Then
        assert_eq!(GameState::GameLoop, state)
    }

    #[test]
    fn run_game_loop() {
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
            game_state: GameState::NewGame,
            player: ghoul,
            enemies: vec![],
            state: State::default(),
        };

        // let mut presenter = MockPresenter::new();
        // presenter.expect_print().with(eq("New game")).once();

        let game_loop = GameLoop;

        // When
        let state = game_loop.game_loop(&mut game);

        // Then
        assert_eq!(GameState::GameOver, state)
    }

    #[test]
    fn run_game_over() {
        // Given
        let mut presenter = MockPresenter::new();
        presenter
            .expect_print()
            .with(eq("Game over"))
            .once()
            .returning(|_| ());

        let game_loop = GameLoop;

        // When
        let state = game_loop.game_over(&presenter);

        // Then
        assert_eq!(GameState::NewGame, state)
    }
}
