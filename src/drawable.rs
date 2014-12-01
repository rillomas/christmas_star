pub trait Drawable {
    fn draw(&self) -> Result<(),String>;
}