pub trait Screen {
    fn draw(&self);
    fn update(&mut self);
}