use tetra::ContextBuilder;

mod game_state;
mod entity;
mod blocks;

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Rustanoid", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(game_state::GameState::new)
}


