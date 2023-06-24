use ggez::{graphics::Canvas, Context};

pub trait RenderGame {
    fn new(ctx: &mut Context) -> Self;
    fn draw(&mut self, canvas: &mut Canvas);
    fn update(&mut self, ctx: &mut Context);
}
