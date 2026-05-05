use std::collections::HashMap;

pub type WindowId = u32;

#[derive(Debug)]
pub struct Window {
    pub id: WindowId,
    pub mapped: bool,
}

pub struct WindowManager {
    windows: HashMap<WindowId, Window>,
    next_id: WindowId,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_window(&mut self) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;

        self.windows.insert(id, Window { id, mapped: false });
        id
    }
}