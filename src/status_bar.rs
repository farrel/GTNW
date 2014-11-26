extern crate ncurses;
extern crate time;

pub struct StatusBar {
    pub window: ncurses::WINDOW,
    pub defcon: i8
}

fn current_time() -> String {
    time::strftime("%d/%m/%Y %H:%M:%S",&time::now())
}

impl StatusBar {
    pub fn new() -> StatusBar {
        StatusBar { window: ncurses::newwin(1,ncurses::getmaxx(ncurses::stdscr),ncurses::getmaxy(ncurses::stdscr) - 2,0),
        defcon: 5 } 
    }

    pub fn draw(&self) {
        ncurses::wattr_on(self.window,ncurses::A_REVERSE());

        self.draw_bar();
        self.draw_defcon();
        self.draw_date();

        ncurses::wattr_off(self.window,ncurses::A_REVERSE());
        ncurses::wrefresh(self.window);
    }

    fn draw_bar(&self) {
        for x in range(0i32, ncurses::getmaxx(self.window)) {
            ncurses::wmove(self.window, 0, x);
            ncurses::waddch(self.window, ' ' as u32);
        }
    }

    fn draw_date(&self) { 
        ncurses::mvwaddstr(self.window,0, ncurses::getmaxx(self.window) - 25, format!("TIME: {}", current_time()).as_slice());
    }

    fn draw_defcon(&self) {
        ncurses::mvwaddstr(self.window, 0, 0, format!("DEFCON {}", self.defcon).as_slice());
    }
}                
