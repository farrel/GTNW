extern crate ncurses;
extern crate gtnw;

use gtnw::display::Display;
use gtnw::status_bar::StatusBar;
use gtnw::command_window::CommandWindow;

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

    let display = Display { window:  ncurses::newwin(max_y - 2,max_x,0,0)};
    display.initialise();

    let command_window = CommandWindow{ window: ncurses::newwin(1,max_x,max_y - 1,0)};
    command_window.initialise();

    let status_bar = StatusBar { window: ncurses::newwin(1,max_x,max_y - 2,0), defcon: 5 };
    status_bar.initialise();

    loop {
        status_bar.draw();
        display.draw("This is a an alert");
    }
    ncurses::endwin();
}
