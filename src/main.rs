use rustyhud::*;
use ncurses::*;
use std::{thread, time};

fn main() {
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

    let mut example_bar = WindowData::new()
        .with_id("Bruv-o-meter")
        .with_content(WindowContent::ProgressBar)
        .with_message("0/20")
        .with_ticks("#")
        .with_position(15, 3)
        .with_dimensions(30, 1)
        .build();

    example_bar.open();

    let mut list = WindowData::new()
        .with_id("Sample list")
        .with_content(WindowContent::List)
        .with_items(vec!["Option a", "Option b", "Option c"])
        .with_position(22, 10)
        .with_dimensions(10, 3)
        .build();

    list.open();

    let mut score_board = WindowData::new()
        .with_id("Sample board")
        .with_content(WindowContent::ScoreBoard)
        .with_items(vec!["Character a:10", "Character b:9", "Character Z-theta-RNQ-B-Cordorey7:8"])
        .with_position(12, 3)
        .with_dimensions(45, 3)
        .build();

    score_board.open();

    thread::sleep(time::Duration::from_millis(5000));

    for i in 0..21 {
        example_bar.message = format!("{}/20", i);
        example_bar.redraw();
        thread::sleep(time::Duration::from_millis(100));
    }

    // Clean up.
    endwin();
    std::process::exit(0);

}
