use rustyhud::*;
use ncurses::*;
use std::{thread, time};
use std::collections::HashMap;


fn main() {

    // Set up omniscient stuff
    // HashMap of active windows to keep track of stuff.
    let mut windows: std::collections::HashMap<String, (WINDOW, WindowData)> = HashMap::new();

    launch();

    // Use WindowData structs to keep track of the windows you want to create

    // Create one window
    let window = WindowData {
            id: String::from("win1"),
            content: WindowContent::Text,
            message: String::from("Hello!!!"),
            style: WindowStyle::Plain,
            ticks: None,
            x_pos: 10,
            y_pos: 10,
            width: 10,
            height: 10,
            priority: false
        };

    open_win(window, &mut windows); // Open the window.
    thread::sleep(time::Duration::from_millis(3000)); // Sleep a bit.

    // Create another window.
    let mod_window = WindowData {
            id: String::from("win2"),
            content: WindowContent::Text,
            message: String::from("I'm gonna change this window."),
            style: WindowStyle::Plain,
            ticks: None,
            x_pos: 25,
            y_pos: 10,
            width: 10,
            height: 10,
            priority: false
        };

    open_win(mod_window, &mut windows); // Open the window
    thread::sleep(time::Duration::from_millis(3000));

    change_window_message("win2", "I HAVE CHANGED YOUUUUUU!!!!", &mut windows);

    thread::sleep(time::Duration::from_millis(1000));

    change_window_style("win2", WindowStyle::Bold, &mut windows);

    thread::sleep(time::Duration::from_millis(1000));

    // How about a progressbar?

    let progress = WindowData {
        id: String::from("BRUH-ometer"),
        content: WindowContent::ProgressBar,
        message: String::from("0/20"),
        style: WindowStyle::Plain,
        ticks: Some(String::from("&")),
        x_pos: 5,
        y_pos: 5,
        width: 30,
        height: 1,
        priority: false
    };

    open_win(progress, &mut windows); // Open the window

    thread::sleep(time::Duration::from_millis(1000));

    for i in 0..21 {
        change_window_message("BRUH-ometer", format!("{}/20", i).as_str(), &mut windows);
        thread::sleep(time::Duration::from_millis(100));
    }

    thread::sleep(time::Duration::from_millis(1000));

    change_window_ticks("BRUH-ometer", "O", &mut windows);

    thread::sleep(time::Duration::from_millis(1000));

    // Clean up.
    endwin();
    std::process::exit(0);

}
