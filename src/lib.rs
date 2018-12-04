extern crate cfg_if;
extern crate wasm_bindgen;
extern crate good;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

struct Points {
    x: Vec<i32>,
    y: Vec<i32>,
}

impl Points {
    fn new() -> Self {
        Points { x: Vec::new(), y: Vec::new() }
    }
}

enum Status {
    Waiting,
    Running,
    Done,
}

struct Config {
    status: Status,
    halt: u8,
}

impl Config {
    fn new() -> Self {
        Config { status: Status::Waiting, halt: 0 }
    }

    fn running(&mut self) -> bool {
        if let Status::Waiting = self.status {
            self.status = Status::Running;
            true
        }
        else {
            false
        }
    }
}

pub struct Input {
    points: Points,
}

impl Input {
    fn new() -> Self {
        Input { points: Points::new() }
    }
}

#[wasm_bindgen]
pub struct Core {
    config: Config,
    input: Input,
}

#[wasm_bindgen]
impl Core {
    pub fn new() -> Self {
        Core { config: Config::new(), input: Input::new() }
    }

    pub fn input_add_point(&mut self, x: i32, y: i32) {
        self.input.points.x.push(x);
        self.input.points.y.push(y);
    }

    pub fn input_length(&self) -> u32 {
        self.input.points.x.len() as u32
    }

    pub fn input_points_x(&self) -> *const i32 {
        self.input.points.x.as_ptr()
    }

    pub fn input_points_y(&self) -> *const i32 {
        self.input.points.y.as_ptr()
    }

    pub fn halt(&self) -> *const u8 {
        &self.config.halt
    }

    pub fn start(&mut self) -> bool {
        if self.config.running() {
            let mut points = Vec::new();
            self.input.points.x.iter().enumerate().for_each(|x| points.push((*x.1, self.input.points.y[x.0])));
            good::run(points).last().unwrap().0;
            true
        }
        else {
            false
        }
    }
}
