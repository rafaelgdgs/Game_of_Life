use macroquad::prelude::*;
use std::{thread, time};

// if LIVE and neighbours < 2 = DIE
// if LIVE and 2 <= neighbours <= 3 = LIVE
// if LIVE and neighbours > 3 = DIE
// if DEAD and neighbours == 3 = LIVE

#[macroquad::main("Game of Life")]
async fn main() {
    println!("Hello, world!");
    loop {
        println!("FPS: {:.1}", get_fps());
        thread::sleep(time::Duration::from_millis(15));
        next_frame().await
    }
}
