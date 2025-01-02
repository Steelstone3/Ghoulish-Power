#[allow(dead_code)]
#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub enum State {
    #[default]
    NewGame,
    GameLoop,
    GameOver,
}
