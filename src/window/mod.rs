use smithay::wayland::shell::xdg::XdgToplevelSurface;
use crate::state::WindowId;

pub struct Window {
    pub id: WindowId,
    pub surface: XdgToplevelSurface,
}
