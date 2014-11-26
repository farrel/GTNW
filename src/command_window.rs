extern crate ncurses;

pub struct CommandWindow {
    pub window: ncurses::WINDOW
}

impl CommandWindow {
    pub fn new() -> CommandWindow {
        let command_window = CommandWindow { window: ncurses::newwin(1, ncurses::getmaxx(ncurses::stdscr), ncurses::getmaxy(ncurses::stdscr) - 1, 0) };

        ncurses::waddstr(command_window.window,">>");
        ncurses::wrefresh(command_window.window);

        command_window
    }

    pub fn initialise(&self) {
    }

    pub fn draw(&self){
    }

//    pub fn get_command(&self) -> &str {
//        let mut string = "".to_string();
//        ncurses::wmove(self.window, 0, 4);
//        ncurses::getstr(&string);
//        string.as_slice()
//    }
}
