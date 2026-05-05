use std::collections::HashMap;
use crate::window::WindowId;

pub struct FloatingLayout {
    windows: HashMap<WindowId, (i32, i32)>,
}

impl FloatingLayout {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn add_window(&mut self, id: WindowId) {
        self.windows.insert(id, (100, 100));
    }
}