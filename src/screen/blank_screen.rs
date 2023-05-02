use crate::screen::Screen;

pub struct BlankScreen;

impl Screen for BlankScreen {
    fn draw(&self) {
    }

    fn update(&mut self) {
    }
}