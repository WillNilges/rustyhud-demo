use rustyhud::*;
use ncurses::*;
use std::{thread, time};
use std::collections::HashMap;


fn main() {

    // Set up omniscient stuff
    // HashMap of active windows to keep track of stuff.
    let mut windows: std::collections::HashMap<String, (WINDOW, WindowData)> = HashMap::new();

    launch();
    let mut testwin2 = WindowData::new()
        .with_id("yeetussss")
        .with_message("Hallo")
        .with_position(20, 20)
        .with_dimensions(20, 20)
        .build();
        
    testwin2.open();
    
    thread::sleep(time::Duration::from_millis(500));

    testwin2.close();

    thread::sleep(time::Duration::from_millis(500));

    testwin2.open();

    thread::sleep(time::Duration::from_millis(500));

    testwin2.mv(30, 13);

    let mut otherwin = WindowData::new()
        .with_id("Changewin")
        .with_message("I'm gonna change this.")
        .with_position(10, 10)
        .with_dimensions(13, 13)
        .build();

    otherwin.open();

    thread::sleep(time::Duration::from_millis(500));

    testwin2.resize(8, 8);

    otherwin.message = String::from("I changed it!");
    otherwin.redraw();

    thread::sleep(time::Duration::from_millis(2000));

    let mut progressbaryeetus = WindowData::new()
        .with_id("Bruv-o-meter")
        .with_content(WindowContent::ProgressBar)
        .with_message("0/20")
        .with_ticks("#")
        .with_position(15, 3)
        .with_dimensions(30, 1)
        .build();

    progressbaryeetus.open();

    for i in 0..21 {
        progressbaryeetus.message = format!("{}/20", i);
        progressbaryeetus.redraw();
        thread::sleep(time::Duration::from_millis(100));
    }

    // Clean up.
    endwin();
    std::process::exit(0);

}
