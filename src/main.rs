#![feature(globs)]

extern crate ncurses;
use ncurses::*;

fn main() {
    initscr();
    noecho();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;

    getmaxyx(stdscr, &mut max_y, &mut max_x);

    let main_window  = newwin(max_y - 2,max_x,0,0);
    let status_bar   = newwin(1,max_x,max_y - 2,0);
    let input_window = newwin(1,max_x,max_y - 1,0);

    waddstr(main_window, "MAIN");
    waddstr(status_bar, "STATUS");
    waddstr(input_window,"INPUT");

    wrefresh(main_window);
    wrefresh(status_bar);
    wrefresh(input_window);

    std::io::timer::sleep(std::time::duration::Duration::minutes(1));

    endwin();
}
