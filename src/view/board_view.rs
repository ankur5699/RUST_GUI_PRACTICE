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

        canvas.set_draw_color(Color::RGB(0, 120, 0));
        let point1 = sdl2::rect::Point::new(32, 32);
        let point2 = sdl2::rect::Point::new(32, 512);

        canvas
            .draw_line(point1, point2)
            .expect("You have done Something Wrong !!")
    }
}
