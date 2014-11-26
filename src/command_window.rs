extern crate ncurses;

use ncurses::*;

pub struct CommandWindow {
    pub window: WINDOW
}

impl CommandWindow {
    pub fn new() -> CommandWindow {
        let command_window = CommandWindow { window: newwin(1, getmaxx(stdscr), getmaxy(stdscr) - 1, 0) };

        command_window.draw();
        command_window
    }

    pub fn draw(&self){
        wmove(self.window, 0, 0);
        waddstr(self.window,">>");
        wclrtoeol(self.window);
        wrefresh(self.window);
    }

    pub fn get_command(&self) {
        let string = &mut String::new();
        wmove(self.window, 0, 3);
        wrefresh(self.window);
        wgetstr(self.window,string);
        self.draw();
    }
}
