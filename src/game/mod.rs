pub(crate) mod entities;
pub(crate) mod on_event;
pub(crate) mod rendering;
pub(crate) mod state;
pub(crate) mod world;

use ggez::{GameResult, conf::WindowSetup, event};
use state::MainState;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn run() -> GameResult {
    let window_title = "Rust game";
    let cb = ggez::ContextBuilder::new(CRATE_NAME, "me")
        .window_setup(WindowSetup::default().title(window_title).vsync(true))
        .add_resource_path("./resources");

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
