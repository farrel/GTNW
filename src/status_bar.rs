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
    pub fn initialise(&self) {
    }

    pub fn draw(&self) {
        self.draw_defcon();
        self.draw_date();
        wrefresh(self.window);
    }

    fn draw_date(&self) { 
    }

    fn draw_defcon(&self) {
        mvwaddstr(self.window, 0,0, format!("DEFCON {} {}", self.defcon, current_time() ).as_slice());
    }
}
