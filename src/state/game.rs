use mockall::automock;

use super::{
    game_loop::GameLooper,
    states::{GameState, State},
};
use crate::{
    models::soldier::ghoul::Ghoul,
    presenters::{console::Console, ghoul_presenter::GhoulPresenter},
};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct GhoulishPower {
    pub game_state: GameState,
    pub state: State,
    pub player: Ghoul,
    pub enemies: Vec<Ghoul>,
}

impl GhoulishPower {
    pub fn new(ghoul_presenter: &dyn GhoulPresenter) -> Self {
        Self {
            game_state: Default::default(),
            state: State::default(),
            player: Ghoul::new(ghoul_presenter),
            enemies: vec![],
        }
    }
}

impl Game for GhoulishPower {
    fn game_loop(&mut self, game_loop: &dyn GameLooper) {
        while self.game_state != GameState::GameOver {
            self.game_state = game_loop.run(self, &Console);
        }
    }
}

#[automock]
pub trait Game {
    fn game_loop(&mut self, game_loop: &dyn GameLooper);
}

#[cfg(test)]
mod game_should {
    use crate::{
        models::soldier::{
            constitution::{armour::GhoulArmour, health::GhoulHealth, mana::GhoulMana},
            elements::Element,
            ghoul::Ghoul,
            types::{armour_types::ArmourType, ghoul_types::GhoulType, weapon_types::WeaponType},
            weapon::GhoulWeapon,
        },
        presenters::ghoul_presenter::MockGhoulPresenter,
        state::{
            game::GhoulishPower,
            game_loop::MockGameLooper,
            states::{GameState, State},
        },
    };
    use mockall::predicate::eq;

    use super::Game;

    #[test]
    fn construct_player() {
        // Given
        let state = GameState::NewGame;

        let ghoul_type = GhoulType::Undead;

        let health = GhoulHealth { health: 100 };

        let armour_type = ArmourType::FullPlate;
        let armour_element = Element::Fire;
        let armour = GhoulArmour {
            armour: 100,
            armour_type,
            armour_element,
        };

        let mana = GhoulMana { mana: 100 };

        let weapon_type = WeaponType::Sword;
        let weapon_element = Element::Fire;
        let weapon = GhoulWeapon {
            weapon_type,
            weapon_element,
            damage: 5..10,
        };

        let player = Ghoul {
            ghoul_type,
            health,
            armour,
            mana,
            weapon,
        };

        let mut ghoul_presenter = MockGhoulPresenter::new();
        ghoul_presenter
            .expect_select_ghoul_type()
            .once()
            .return_const(ghoul_type);
        ghoul_presenter
            .expect_select_weapon_type()
            .once()
            .return_const(weapon_type);
        ghoul_presenter
            .expect_select_armour_type()
            .once()
            .return_const(armour_type);
        ghoul_presenter
            .expect_select_element()
            .with(eq("armour".to_string()))
            .once()
            .return_const(armour_element);
        ghoul_presenter
            .expect_select_element()
            .with(eq("weapon".to_string()))
            .once()
            .return_const(weapon_element);

        let expected_ghoulish_power = GhoulishPower {
            game_state: state,
            player,
            enemies: vec![],
            state: State::default(),
        };

        // When
        let ghoulish_power = GhoulishPower::new(&ghoul_presenter);

        // Then
        assert_eq!(expected_ghoulish_power, ghoulish_power);
    }

    #[test]
    fn run_game_loop_game_over() {
        // Given
        let mut ghoulish_power = GhoulishPower {
            game_state: GameState::GameOver,
            player: Ghoul {
                ghoul_type: GhoulType::Undead,
                health: GhoulHealth { health: 100 },
                armour: GhoulArmour {
                    armour: 100,
                    armour_type: ArmourType::FullPlate,
                    armour_element: Element::Air,
                },
                mana: GhoulMana { mana: 100 },
                weapon: GhoulWeapon {
                    weapon_type: WeaponType::Sword,
                    weapon_element: Element::Air,
                    damage: 5..10,
                },
            },
            enemies: vec![],
            state: State::default(),
        };

        let mut game_loop = MockGameLooper::new();
        game_loop.expect_run().never();

        // When
        ghoulish_power.game_loop(&game_loop);
    }

    #[test]
    fn run_game_loop_new_game() {
        // Given
        let mut ghoulish_power = GhoulishPower {
            game_state: GameState::NewGame,
            player: Ghoul {
                ghoul_type: GhoulType::Undead,
                health: GhoulHealth { health: 100 },
                armour: GhoulArmour {
                    armour: 100,
                    armour_type: ArmourType::FullPlate,
                    armour_element: Element::Air,
                },
                mana: GhoulMana { mana: 100 },
                weapon: GhoulWeapon {
                    weapon_type: WeaponType::Sword,
                    weapon_element: Element::Air,
                    damage: 5..10,
                },
            },
            enemies: vec![],
            state: State::default(),
        };

        let mut game_loop = MockGameLooper::new();
        game_loop
            .expect_run()
            .return_const(GameState::GameOver)
            .once();

        // When
        ghoulish_power.game_loop(&game_loop);
    }
}
