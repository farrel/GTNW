extern crate std;
extern crate ncurses;

use ncurses::*;

pub struct Display {
    pub window: WINDOW
}

impl Display {

    pub fn new() -> Display {
        let display = Display { window: newwin(getmaxy(stdscr) - 2,getmaxx(stdscr),0,0) };

        scrollok(display.window, true);
        wmove(display.window, getmaxy(display.window) - 1, 0);

        display
    }

    pub fn draw(&self, text: &str) {
        for character in text.chars(){
            std::io::timer::sleep(std::time::duration::Duration::milliseconds(50));
            waddch(self.window, character as u32);
            wrefresh(self.window);
        }
        wprintw(self.window, "\n");
        wrefresh(self.window);
    }

    pub fn draw_reverse(&self, text: &str) {
        wattr_on(self.window, A_REVERSE());
        self.draw(text);
        wattr_off(self.window, A_REVERSE());
    }
}

