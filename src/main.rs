use macroquad::prelude::*;
use std::{thread, time};

// if LIVE and neighbours < 2 = DIE
// if LIVE and 2 <= neighbours <= 3 = LIVE
// if LIVE and neighbours > 3 = DIE
// if DEAD and neighbours == 3 = LIVE

const GRID_SIZE: usize = 40;
const CELL_SIZE: f32 = 10f32;

trait Draw {
    fn draw(&self);
}

#[derive(PartialEq)]
enum CellState {
    Live,
    Dead,
}

impl CellState {
    fn process(&self, neighbours: u8) -> CellState {
        match self {
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

struct Cell {
    state: CellState,
    pos: Vec2,
}

impl Draw for Cell {
    fn draw(&self) {
        let color = match self.state {
            CellState::Live => BLACK,
            CellState::Dead => WHITE,
        };
        draw_rectangle_lines(
            self.pos.x - 1f32,
            self.pos.y - 1f32,
            CELL_SIZE + 2f32,
            CELL_SIZE + 2f32,
            2f32,
            GRAY,
        );
        draw_rectangle(self.pos.x, self.pos.y, CELL_SIZE, CELL_SIZE, color);
    }
}

struct Game {
    grid_current: Vec<Cell>,
    paused: bool,
}

impl Game {
    fn new() -> Self {
        let mut grid: Vec<Cell> = Vec::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let cell = Cell {
                    state: CellState::Dead,
                    pos: Vec2 {
                        x: (j * 10) as f32,
                        y: (i * 10) as f32,
                    },
                };
                grid.push(cell);
            }
        }
        Self {
            grid_current: grid,
            paused: false,
        }
    }

    fn run(&mut self) {
        draw_text(
            format!("{}, {}", mouse_position().0, mouse_position().1).as_str(),
            500f32,
            600f32,
            30f32,
            BLACK,
        );
        if is_mouse_button_pressed(MouseButton::Right) {
            self.paused = !self.paused;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position();
            let cell = self.grid_current.get_mut(Game::coords_to_index(pos));
            match cell {
                None => {}
                Some(x) => {
                    x.state = match x.state {
                        CellState::Dead => CellState::Live,
                        CellState::Live => CellState::Dead,
                    };
                }
            }
        }
        if self.paused == true {
            draw_text("PAUSED", 500f32, 500f32, 30f32, BLACK);
            return;
        }
        self.compute_tick();
    }

    fn compute_tick(&mut self) {
        let mut grid_next_frame: Vec<Cell> = Vec::new();
        for line in 0..GRID_SIZE {
            for column in 0..GRID_SIZE {
                let cell_pos = match self.grid_current.get(line * GRID_SIZE + column) {
                    None => {
                        continue;
                    }
                    Some(v) => v,
                };
                let c: Cell = Cell {
                    state: cell_pos.state.process(self.get_neighbours(cell_pos)),
                    pos: cell_pos.pos,
                };
                grid_next_frame.push(c);
            }
        }
        self.grid_current = grid_next_frame;
    }

    fn coords_to_index(coords: (f32, f32)) -> usize {
        ((coords.1 / CELL_SIZE).floor() * GRID_SIZE as f32 + (coords.0 / CELL_SIZE).floor())
            as usize
    }

    fn get_neighbours(&self, cell: &Cell) -> u8 {
        let coords = cell.pos;
        let mut neighbours_count: u8 = 0;
        let shifts: [(i32, i32); 8] = [
            (10, 0),
            (10, 10),
            (0, 10),
            (-10, 10),
            (-10, 0),
            (-10, -10),
            (0, -10),
            (10, -10),
        ];

        for shift in shifts {
            if coords.x + shift.0 as f32 > GRID_SIZE as f32 * CELL_SIZE
                || (coords.x + shift.0 as f32) < 0f32
                || coords.y + shift.1 as f32 > GRID_SIZE as f32 * CELL_SIZE
                || (coords.y + shift.1 as f32) < 0f32
            {
                continue;
            }
            match self.grid_current.get(Game::coords_to_index((
                coords.x + shift.0 as f32,
                coords.y + shift.1 as f32,
            ))) {
                None => {}
                Some(c) => {
                    if c.state == CellState::Live {
                        neighbours_count += 1
                    }
                }
            }
        }
        neighbours_count
    }
}

impl Draw for Game {
    fn draw(&self) {
        for cell in &self.grid_current {
            cell.draw();
        }
    }
}

#[macroquad::main("Game of Life")]
async fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    loop {
        clear_background(WHITE);
        game.run();
        game.draw();
        println!("FPS: {:.1}", get_fps());
        thread::sleep(time::Duration::from_millis(100));
        next_frame().await
    }
}
