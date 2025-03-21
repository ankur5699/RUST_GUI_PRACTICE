use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas
            .fill_rect(self.screen_area)
            .expect("Something Catastrphic happened");
    }
}
