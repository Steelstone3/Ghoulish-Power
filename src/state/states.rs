#[allow(dead_code)]
#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub enum GameState {
    #[default]
    NewGame,
    GameLoop,
    GameOver,
}

#[allow(dead_code)]
#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub enum State {
    Exploration,
    #[default]
    Settlement,
    PlayerTurn,
    EnemyTurn,
}
