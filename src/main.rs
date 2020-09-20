use rustyhud;
use ncurses::*;
use std::{thread, time};
use std::collections::HashMap;

fn main() {

    // Set up omniscient stuff
    // HashMap of active windows to keep track of stuff.
    let mut windows: std::collections::HashMap<String, (WINDOW, rustyhud::WindowData)> = HashMap::new();

    rustyhud::launch();

    // Use WindowData structs to keep track of the windows you want to create

    // Create one window
    let window = rustyhud::WindowData {
            id: "win1".to_string(),
            content: rustyhud::WindowContent::Text,
            message: "Hello!!!".to_string(),
            style: rustyhud::WindowStyle::Plain,
            x_pos: 10,
            y_pos: 10,
            width: 10,
            height: 10,
            priority: false
        };
    rustyhud::open_win(window, &mut windows); // Open the window.
    thread::sleep(time::Duration::from_millis(3000)); // Sleep a bit.

    // Create another window.
    let mod_window = rustyhud::WindowData {
            id: "win2".to_string(),
            content: rustyhud::WindowContent::Text,
            message: "I'm gonna change this window.".to_string(),
            style: rustyhud::WindowStyle::Plain,
            x_pos: 25,
            y_pos: 10,
            width: 10,
            height: 10,
            priority: false
        };

    rustyhud::open_win(mod_window, &mut windows); // Open the window
    thread::sleep(time::Duration::from_millis(3000));

    rustyhud::change_window_message("win2".to_string(), "I HAVE CHANGED YOUUUUUU!!!!".to_string(), &mut windows);

    thread::sleep(time::Duration::from_millis(1000));


    rustyhud::change_window_style("win2".to_string(), rustyhud::WindowStyle::Bold, &mut windows);

    thread::sleep(time::Duration::from_millis(1000));

    // mod_window.message = "ooooop".to_string(); // Change the name of the window
    // mod_window.priority = true.to_string(); // and make it override the current window.
    //
    // rustyhud::open_win(mod_window, &mut windows); // Open the window

    // Clean up.
    endwin();
    std::process::exit(0);

}
