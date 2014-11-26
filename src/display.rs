extern crate std;
extern crate ncurses;

pub struct Display {
    pub window: ncurses::WINDOW
}

impl Display {

    pub fn new() -> Display {
        let display = Display { window: ncurses::newwin(ncurses::getmaxy(ncurses::stdscr) - 2,ncurses::getmaxx(ncurses::stdscr),0,0) };

        ncurses::scrollok(display.window, true);
        ncurses::wmove(display.window, ncurses::getmaxy(display.window) - 1, 0);

        display
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

    pub fn draw_reverse(&self, text: &str) {
        ncurses::wattr_on(self.window, ncurses::A_REVERSE());
        self.draw(text);
        ncurses::wattr_off(self.window, ncurses::A_REVERSE());
    }
}

