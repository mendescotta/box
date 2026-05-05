pub struct InputManager;

impl InputManager {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_key(&self, key: &str) {
        println!("Key pressed: {}", key);
    }
}