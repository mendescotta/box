mod state;
mod window;
mod input;
mod render;
mod layout;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let mut state = state::State::new()?;
    state.run()
}