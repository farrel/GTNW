extern crate ncurses;
extern crate gtnw;

fn initialise_ncurses() {
    ncurses::initscr();
    ncurses::noecho();
}

fn main() {
    initialise_ncurses();


    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;

    ncurses::getmaxyx(ncurses::stdscr, &mut max_y, &mut max_x);

    let display = gtnw::display::Display { window:  ncurses::newwin(max_y - 2,max_x,0,0), height: max_y - 2, width: max_x };
    display.initialise();

    let input_window = ncurses::newwin(1,max_x,max_y - 1,0);

    let status_bar_window   = ncurses::newwin(1,max_x,max_y - 2,0);
    let status_bar = gtnw::status_bar::StatusBar { window: status_bar_window, defcon: 5 };
    status_bar.initialise();


    ncurses::waddstr(input_window,">>");
    ncurses::wrefresh(input_window);


    for x in range(0i32, 200i32) {
        status_bar.draw();
        display.draw("This is a an alert");
    }
    ncurses::endwin();
}
