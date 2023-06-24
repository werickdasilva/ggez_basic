pub mod render_game;
mod show_fps_screen;

use std::{env, path::PathBuf};

use ggez::{
    conf,
    event::{self, EventHandler},
    graphics::{Canvas, Color},
    Context, ContextBuilder, GameError,
};
use render_game::RenderGame;
use show_fps_screen::ShowFpsScreen;

pub struct State {
    show_fps_screen: ShowFpsScreen,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            show_fps_screen: ShowFpsScreen::new(ctx),
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.show_fps_screen.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(125, 249, 255));
        self.show_fps_screen.draw(&mut canvas);
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        PathBuf::from("./res")
    };

    let conf = conf::Conf::new();

    let (mut ctx, event_loop) = ContextBuilder::new("ggez_basic", "Werick")
        .default_conf(conf)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let state = State::new(&mut ctx);

    event::run(ctx, event_loop, state)
}
