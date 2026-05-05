use std::collections::HashMap;
use smithay::{
    backend::winit,
    reexports::wayland_server::{Display, DisplayHandle},
    wayland::{
        compositor::{CompositorHandler, CompositorState},
        shell::xdg::{XdgShellHandler, XdgShellState, XdgToplevelSurface, XdgRequest},
    },
};
use crate::layout::FloatingLayout;
use crate::window::Window;

pub type WindowId = u32;

pub fn run() -> anyhow::Result<()> {
    let display = Display::new()?;
    let dh = display.handle();
    let backend = winit::init::<State>()?;
    let mut state = State::new(display, dh, backend);
    state.backend.run(state)?;
    Ok(())
}

pub struct State {
    pub display: Display<Self>,
    pub dh: DisplayHandle,
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub windows: HashMap<WindowId, Window>,
    pub next_id: WindowId,
    pub layout: FloatingLayout,
    pub backend: winit::WinitBackend,
}

impl State {
    pub fn new(display: Display<Self>, dh: DisplayHandle, backend: winit::WinitBackend) -> Self {
        let compositor_state = CompositorState::new::<Self>(&dh);
        let xdg_shell_state = XdgShellState::new::<Self>(&dh);
        Self {
            display,
            dh,
            compositor_state,
            xdg_shell_state,
            windows: HashMap::new(),
            next_id: 1,
            layout: FloatingLayout::new(),
            backend,
        }
    }

    pub fn new_window(&mut self, surface: XdgToplevelSurface) {
        let id = self.next_id;
        self.next_id += 1;
        self.windows.insert(id, Window { id, surface });
        self.layout.map(id);
    }
}

impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state(
        &self,
        _client: &smithay::reexports::wayland_server::Client,
    ) -> &smithay::wayland::compositor::CompositorClientState {
        unreachable!()
    }
}

impl XdgShellHandler for State {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, _dh: &DisplayHandle, surface: XdgToplevelSurface) {
        self.new_window(surface);
    }

    fn request(
        &mut self,
        _dh: &DisplayHandle,
        _surface: &XdgToplevelSurface,
        _req: XdgRequest,
    ) {}
}
