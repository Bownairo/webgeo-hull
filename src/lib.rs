extern crate cfg_if;
extern crate wasm_bindgen;

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
    input: Input,
}

#[wasm_bindgen]
impl Core {
    pub fn new() -> Self {
        Core { input: Input::new() }
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
}
