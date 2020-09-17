pub mod entities {
    use embedded_graphics::{pixelcolor::Rgb565, DrawTarget};
use tinytga::{Tga};

use embedded_graphics::prelude::*;


pub struct Wolf<'a> {
    clear: Rgb565,
    image: Tga<'a>,
    offset: Point,
}

impl<'a> Wolf<'a> {
    pub fn new() -> Self {
        Wolf {
            image: Tga::from_slice(include_bytes!("../../assets/wolfleft.tga")).unwrap(),
            clear: Rgb565::new(0, 0, 0),
            offset: Point::new(92, 58),
        }
    }
    pub fn draw<DT>(&self, display: &mut DT)  where DT: DrawTarget<Rgb565> {
        let it = self.image.pixel_iter();

        for p in it {
            if p.1 != self.clear {
                Pixel(p.0 + self.offset, p.1).draw(display);
            }
        }
    }
    pub fn translate(&mut self, x: i32, y: i32) {
        self.offset.y += y;
        self.offset.x += x;
    }
}

}