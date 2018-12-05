extern crate cfg_if;
extern crate wasm_bindgen;
extern crate good;
extern crate webgeo_output;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use webgeo_output::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

enum Status {
    Waiting,
    Running,
    Done,
}

struct Config {
    status: Status,
}

impl Config {
    fn new() -> Self {
        Config { status: Status::Waiting }
    }

    fn run(&mut self) -> bool {
        if let Status::Waiting = self.status {
            self.status = Status::Running;
            true
        }
        else {
            false
        }
    }

    fn end(&mut self) {
        if let Status::Running = self.status {
            self.status = Status::Done;
        }
    }
}

struct Input {
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
    output: Output,
}

#[wasm_bindgen]
impl Core {
    pub fn new() -> Self {
        Core { config: Config::new(), input: Input::new(), output: Output::new() }
    }

    pub fn input_add_point(&mut self, x: i32, y: i32) {
        self.input.points.add(x, y);
    }

    pub fn input_length(&self) -> u32 {
        self.input.points.len()
    }

    pub fn input_points_x(&self) -> *const i32 {
        self.input.points.x()
    }

    pub fn input_points_y(&self) -> *const i32 {
        self.input.points.y()
    }

    pub fn points_length(&self) -> u32 {
        self.output.points_len()
    }

    pub fn points_x(&self) -> *const i32 {
        self.output.points_x()
    }

    pub fn points_y(&self) -> *const i32 {
        self.output.points_y()
    }

    pub fn segs_length(&self) -> u32 {
        self.output.segs_len()
    }

    pub fn segs_x(&self) -> *const i32 {
        self.output.segs_x()
    }

    pub fn segs_y(&self) -> *const i32 {
        self.output.segs_y()
    }

    pub fn rays_length(&self) -> u32 {
        self.output.rays_len()
    }

    pub fn rays_x(&self) -> *const i32 {
        self.output.rays_x()
    }

    pub fn rays_y(&self) -> *const i32 {
        self.output.rays_y()
    }

    pub fn state(&self) -> u8 {
        match self.config.status {
            Status::Waiting => 0,
            Status::Running => 1,
            Status::Done => 2,
        }
    }

    pub fn start(&mut self) {
        if self.config.run() {
            let points = self.input.points.condense();
            // XXX Break output into it's own thing, then use output in good.
            good::run(points, &mut self.output);
        }
    }

    pub fn step(&mut self) {
        self.output.step();
        if self.output.done() {
            self.config.end();
        }
    }
}
