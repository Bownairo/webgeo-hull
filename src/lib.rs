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

    fn add(&mut self, x: i32, y: i32) {
        self.x.push(x);
        self.y.push(y);
    }

    fn len(&self) -> u32 {
        self.x.len() as u32
    }
}

struct Segs {
    x: Vec<i32>,
    y: Vec<i32>,
}

impl Segs {
    fn new() -> Self {
        Segs { x: Vec::new(), y: Vec::new() }
    }

    fn add(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.x.push(x1);
        self.x.push(x2);
        self.y.push(y1);
        self.y.push(y2);
    }

    fn len(&self) -> u32 {
        (self.x.len() / 2) as u32
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

pub struct Input {
    points: Points,
}

impl Input {
    fn new() -> Self {
        Input { points: Points::new() }
    }
}

pub struct Output {
    points: Points,
    segs: Segs,
    rays: Segs,
    stepping: bool,
    halt: u8,
}

impl Output {
    fn new() -> Self {
        Output { points: Points::new(), segs: Segs::new(), rays: Segs::new(), stepping: false, halt: 0 }
    }

    fn stepping(&self) -> bool {
        self.stepping
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
        self.input.points.x.as_ptr()
    }

    pub fn input_points_y(&self) -> *const i32 {
        self.input.points.y.as_ptr()
    }

    pub fn points_length(&self) -> u32 {
        self.output.points.len()
    }

    pub fn points_x(&self) -> *const i32 {
        self.output.points.x.as_ptr()
    }

    pub fn points_y(&self) -> *const i32 {
        self.output.points.y.as_ptr()
    }

    pub fn segs_length(&self) -> u32 {
        self.output.segs.len()
    }

    pub fn segs_x(&self) -> *const i32 {
        self.output.segs.x.as_ptr()
    }

    pub fn segs_y(&self) -> *const i32 {
        self.output.segs.y.as_ptr()
    }

    pub fn rays_length(&self) -> u32 {
        self.output.rays.len()
    }

    pub fn rays_x(&self) -> *const i32 {
        self.output.rays.x.as_ptr()
    }

    pub fn rays_y(&self) -> *const i32 {
        self.output.rays.y.as_ptr()
    }

    pub fn halt(&self) -> *const u8 {
        &self.output.halt
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
            let mut points = Vec::new();
            self.input.points.x.iter().enumerate().for_each(|x| points.push((*x.1, self.input.points.y[x.0])));
            // XXX Break output into it's own thing, then use output in good.
            good::run(points, &mut self.output);
            self.config.end();
        }
    }
}
