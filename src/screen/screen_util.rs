
#[macro_export]
macro_rules! screen_width {
    () => {
        getmaxx(stdscr())
    };
}