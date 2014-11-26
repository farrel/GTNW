extern crate ncurses;
extern crate gtnw;

use gtnw::display::Display;
use gtnw::status_bar::StatusBar;
use gtnw::command_window::CommandWindow;

fn initialise_ncurses() {
    ncurses::initscr();
    //ncurses::noecho();
}

fn main() {
    initialise_ncurses();


    /* Get the screen bounds. */

    let display = Display::new();
    let command_window = CommandWindow::new();
    let status_bar = StatusBar::new();

    loop {
        status_bar.draw();
        display.draw_reverse("ALERT");
        display.draw("This is a an alert");
        command_window.get_command();
        status_bar.draw();
    }
    ncurses::endwin();
}
