use rustyhud;
use ncurses::*;
use std::{thread, time};
use std::collections::HashMap;

fn main() {

    // Set up omniscient stuff
    // HashMap of active windows to keep track of stuff.
    let mut windows: std::collections::HashMap<String, (WINDOW, rustyhud::WindowData)> = HashMap::new();

    rustyhud::launch();
    let window = rustyhud::WindowData {
            id: "win1".to_string(), content: "T".to_string(), message: "Hello!!!".to_string(), style: "plain".to_string(), x_pos: 10, y_pos: 10, width: 10, height: 10, priority: false
        };
    rustyhud::open_win(window, &mut windows);
    refresh();
    thread::sleep(time::Duration::from_millis(5000));
    endwin();
    std::process::exit(0);

}
