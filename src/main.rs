mod state;
mod layout;
mod input;
mod render;
mod backend;
mod window;
mod config;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    state::run()
}
