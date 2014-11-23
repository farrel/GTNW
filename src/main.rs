#![feature(globs)]
extern crate ncurses;
extern crate time;

use ncurses::*;

mod status_bar;

fn initialise_ncurses() {
    initscr();
    noecho();
}

fn main() {
    initialise_ncurses();


    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;

    getmaxyx(stdscr, &mut max_y, &mut max_x);

    let main_window  = newwin(max_y - 2,max_x,0,0);
    let status_bar_window   = newwin(1,max_x,max_y - 2,0);
    let input_window = newwin(1,max_x,max_y - 1,0);

    let status_bar = status_bar::StatusBar { window: status_bar_window, defcon: 5 };
    status_bar.initialise();

    
    scrollok(main_window, true);

    waddstr(input_window,">>");

    wrefresh(input_window);

    wmove(main_window, max_y - 3,0);
    wrefresh(main_window);

    for x in range(0i32, 200i32) {
        wprintw(main_window, format!("{}\n",x).as_slice());

        wrefresh(main_window);
        status_bar.draw();
        std::io::timer::sleep(std::time::duration::Duration::seconds(1));
    }
    endwin();
}
