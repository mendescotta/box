use smithay::{
    reexports::wayland_server::Display,
};

use crate::{window::WindowManager, layout::FloatingLayout};

pub struct State {
    pub display: Display<Self>,
    pub wm: WindowManager,
    pub layout: FloatingLayout,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let display = Display::new()?;
        Ok(Self {
            display,
            wm: WindowManager::new(),
            layout: FloatingLayout::new(),
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("Running compositor...");
        Ok(())
    }
}