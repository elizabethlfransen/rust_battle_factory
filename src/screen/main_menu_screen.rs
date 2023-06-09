use ncurses::*;
use crate::screen::screen::Screen;
use crate::screen_width;

pub struct MainMenuScreen {
    count: i32
}

impl MainMenuScreen {
    pub(crate) fn new() -> Self {
        return MainMenuScreen {
            count: 0
        }
    }
}
impl Screen for MainMenuScreen {
    fn draw(&self) {
        let title = format!("Welcome to battle factory {}", self.count);
        mvaddstr(getmaxy(stdscr()) / 2, (screen_width!() - title.len() as i32) / 2, &title);
    }

    fn update(&mut self) {
        self.count += 1;
    }
}