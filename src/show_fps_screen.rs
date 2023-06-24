use ggez::{
    glam::Vec2,
    graphics::{Canvas, FontData, Text},
    Context,
};

use crate::render_game::RenderGame;

pub struct ShowFpsScreen {
    frames: u8,
    fps: u32,
}

impl RenderGame for ShowFpsScreen {
    fn new(ctx: &mut Context) -> Self {
        ctx.gfx.add_font(
            "MilkyCow",
            FontData::from_path(ctx, "/fonts/milky_cow/MilkyCow.ttf").unwrap(),
        );
        Self {
            frames: 100,
            fps: 0,
        }
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(
            Text::new(format!("FPS: {}", self.fps))
                .set_font("MilkyCow")
                .set_scale(32.),
            Vec2::new(2., 2.),
        );
    }

    fn update(&mut self, ctx: &mut Context) {
        if (self.frames % 100) == 0 {
            self.fps = ctx.time.fps() as u32;
            self.frames = 0;
        }

        self.frames += 1;
    }
}
