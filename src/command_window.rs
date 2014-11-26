extern crate ncurses;

pub struct CommandWindow {
    pub window: ncurses::WINDOW
}

impl CommandWindow {
    pub fn new() -> CommandWindow {
        let command_window = CommandWindow { window: ncurses::newwin(1, ncurses::getmaxx(ncurses::stdscr), ncurses::getmaxy(ncurses::stdscr) - 1, 0) };

        ncurses::scrollok(command_window.window, true);
        command_window.draw();
        command_window
    }

    pub fn initialise(&self) {
    }

    pub fn draw(&self){
        ncurses::wmove(self.window, 0, 0);
        ncurses::waddstr(self.window,">>");
        ncurses::wclrtoeol(self.window);
        ncurses::wrefresh(self.window);
    }

    pub fn get_command(&self) {
        let string = &mut String::new();
        ncurses::wmove(self.window, 0, 3);
        ncurses::wrefresh(self.window);
        ncurses::wgetstr(self.window,string);
        self.draw();
    }
}
