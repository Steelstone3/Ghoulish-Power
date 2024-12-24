pub struct GameLoop;

impl GameLooper for GameLoop {
    fn run(&self) {
        todo!()
    }
}

#[allow(dead_code)]
pub trait GameLooper {
    fn run(&self);
}
