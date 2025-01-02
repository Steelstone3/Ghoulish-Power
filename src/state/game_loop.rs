use mockall::automock;

pub struct GameLoop;

impl GameLooper for GameLoop {
    fn run(&self) {
        todo!()
    }
}

#[automock]
pub trait GameLooper {
    fn run(&self);
}

#[cfg(test)]
mod game_loop_should {
    #[test]
    #[ignore = "not implemented"]
    fn run() {}
}
