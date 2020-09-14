use rustyhud;
use ncurses::*;
use std::{thread, time};

fn main() {
    rustyhud::launch();
    refresh();
    thread::sleep(time::Duration::from_millis(5000));
    endwin();
    std::process::exit(0);

}
