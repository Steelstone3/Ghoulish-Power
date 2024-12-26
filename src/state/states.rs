#[allow(dead_code)]
#[derive(Default, PartialEq, Debug)]
pub enum State {
    #[default]
    NewGame,
    GameLoop,
    GameOver,
}
