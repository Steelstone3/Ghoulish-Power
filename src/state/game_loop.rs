use mockall::automock;

pub struct GameLoop;

impl GameLooper for GameLoop {
    fn run(&self) {
        todo!()
    }
}

#[allow(dead_code)]
#[automock]
pub trait GameLooper {
    fn run(&self);
}
