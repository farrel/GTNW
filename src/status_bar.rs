extern crate ncurses;
extern crate time;

use ncurses::*;

pub struct StatusBar {
    pub window: WINDOW,
    pub defcon: i8
}

fn current_time() -> String {
    time::strftime("%d/%m/%Y %H:%M:%S",&time::now())
}

impl StatusBar {
    pub fn new() -> StatusBar {
        StatusBar { window: newwin(1,getmaxx(stdscr),getmaxy(stdscr) - 2,0),
        defcon: 5 } 
    }

    pub fn draw(&self) {
        wattr_on(self.window,A_REVERSE());

        self.draw_bar();
        self.draw_defcon();
        self.draw_date();

        wattr_off(self.window,A_REVERSE());
        wrefresh(self.window);
    }

    fn draw_bar(&self) {
        for x in range(0i32, getmaxx(self.window)) {
            wmove(self.window, 0, x);
            waddch(self.window, ' ' as u32);
        }
    }

    fn draw_date(&self) { 
        mvwaddstr(self.window,0, getmaxx(self.window) - 25, format!("TIME: {}", current_time()).as_slice());
    }

    fn draw_defcon(&self) {
        mvwaddstr(self.window, 0, 0, format!("DEFCON {}", self.defcon).as_slice());
    }
}                
