#![feature(globs)]
extern crate ncurses;

use ncurses::*;
use display::Display;
use status_bar::StatusBar;
use command_window::CommandWindow;

mod command_window;
mod display;
mod status_bar;

fn initialise_ncurses() {
    initscr();
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
    endwin();
}
