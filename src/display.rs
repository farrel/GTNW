extern crate std;
extern crate ncurses;

pub struct Display {
    pub window: ncurses::WINDOW
}

impl Display {
    pub fn initialise(&self) {
        ncurses::scrollok(self.window, true);
        ncurses::wmove(self.window, ncurses::getmaxy(self.window) - 1, 0);
    }

    pub fn draw(&self, text: &str) {
        for character in text.chars(){
            std::io::timer::sleep(std::time::duration::Duration::milliseconds(50));
            ncurses::waddch(self.window, character as u32);
            ncurses::wrefresh(self.window);
        }
        ncurses::wprintw(self.window, "\n");
        ncurses::wrefresh(self.window);
    }
}
