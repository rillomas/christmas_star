use control;

/// An object within the game
pub trait Object {
    fn update(&mut self, cs: &control::State) -> Result<(),String>;
    fn draw(&self) -> Result<(),String>;
}
