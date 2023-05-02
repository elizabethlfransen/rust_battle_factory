use ncurses::*;
use crate::screen::{MainMenuScreen, ScreenStack};

fn initialize_ncurses() {
    setlocale(LcCategory::all, "");
    initscr();
    keypad(stdscr(), true);
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

struct Game {
    pub running: bool,
    screens: ScreenStack
}

impl Game {
    fn update(&mut self) {
        if let Some(screen) = self.screens.get_current_screen_as_mut() {
            screen.update();
        }
    }

    fn draw(&self) {
        if let Some(screen) = self.screens.get_current_screen() {
            screen.draw();
        }
    }

    fn new() -> Game {
        return Game {
            running: true,
            screens: ScreenStack::new(MainMenuScreen)
        };
    }
}

pub fn run_game() {
    initialize_ncurses();
    let mut game = Game::new();
    while game.running {
        erase();
        game.draw();
        refresh();
        game.update();
    }
}