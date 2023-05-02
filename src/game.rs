use std::thread;
use std::time::{Duration, SystemTime};
use ncurses::*;
use crate::screen::{MainMenuScreen, ScreenStack};

fn initialize_color() {
    start_color();
    use_default_colors();
    for i in 1..256 {
        init_pair(i,i,232);
    }
    bkgd(COLOR_PAIR(15));
}

fn initialize_ncurses() {
    setlocale(LcCategory::all, "");
    initscr();
    keypad(stdscr(), true);
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    initialize_color();
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
            screens: ScreenStack::new(MainMenuScreen::new())
        };
    }
}

const FRAMES_PER_SECOND: u64 = 30;
const FRAME_DURATION : Duration = Duration::from_millis(1000 / FRAMES_PER_SECOND);

pub fn run_game() {
    initialize_ncurses();
    let mut game = Game::new();
    while game.running {
        let end_time = SystemTime::now();
        erase();
        game.draw();
        refresh();
        game.update();
        let sleep_duration = FRAME_DURATION - end_time.elapsed().unwrap();
        thread::sleep(sleep_duration);
    }
}