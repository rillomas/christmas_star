pub trait Draw {
    fn draw(&self) -> Result<(),String>;
}