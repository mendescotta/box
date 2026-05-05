use std::collections::HashMap;
use smithay::utils::{Logical, Point, Rectangle, Size};
use crate::state::WindowId;

pub struct FloatingLayout {
    positions: HashMap<WindowId, Point<i32, Logical>>,
    sizes: HashMap<WindowId, Size<i32, Logical>>,
}

impl FloatingLayout {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            sizes: HashMap::new(),
        }
    }

    pub fn map(&mut self, window: WindowId) {
        self.positions.insert(window, (100, 100).into());
        self.sizes.insert(window, (800, 600).into());
    }
}
