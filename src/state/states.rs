#[allow(dead_code)]
#[derive(Default)]
pub enum State {
    #[default]
    NewGame,
    GameLoop,
    GameOver,
}
