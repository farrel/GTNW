extern crate ncurses;

pub struct CommandWindow {
    pub window: ncurses::WINDOW
}

impl CommandWindow {
    pub fn initialise(&self) {
        ncurses::waddstr(self.window,">>");
        ncurses::wrefresh(self.window);
    }

    pub fn draw(&self){
    }
}
