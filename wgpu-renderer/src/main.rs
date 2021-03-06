#![windows_subsystem = "windows"]
#![allow(clippy::unreadable_literal)]

mod engine;
mod game_loop;
mod geometry;
mod rendering;

fn main() {
    let mut ctx = engine::Context::new();

    let state = game_loop::State::new(&mut ctx);
    ctx.start(state);
}
