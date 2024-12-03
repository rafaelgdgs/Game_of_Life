use macroquad::prelude::*;
use std::{thread, time};

// if LIVE and neighbours < 2 = DIE
// if LIVE and 2 <= neighbours <= 3 = LIVE
// if LIVE and neighbours > 3 = DIE
// if DEAD and neighbours == 3 = LIVE

enum CellState {
    Live,
    Dead,
}

struct Cell {
    state: CellState,
    x: u32,
    y: u32,
}

impl Cell {
    fn play(&self, neighbours: u8) -> CellState {
        match self.state {
            CellState::Live => match neighbours {
                2 | 3 => CellState::Live,
                _ => CellState::Dead,
            },
            CellState::Dead => match neighbours {
                3 => CellState::Live,
                _ => CellState::Dead,
            },
        }
    }
}

struct Game {
    grid: Vec<Cell>,
}

impl Game {
    fn run(&self) {}
}

#[macroquad::main("Game of Life")]
async fn main() {
    println!("Hello, world!");
    loop {
        println!("FPS: {:.1}", get_fps());
        thread::sleep(time::Duration::from_millis(15));
        next_frame().await
    }
}
