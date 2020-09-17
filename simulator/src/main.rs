use embedded_graphics::pixelcolor::Rgb565;
use shared::entities;
use tinytga::Tga;

use embedded_graphics::image::Image;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::thread;
use std::time::Duration;
extern crate shared;
fn main() {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(160, 128));
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("Example", &output_settings);

    let tga = Tga::from_slice(include_bytes!("../../assets/ground.tga")).unwrap();
    let image = Image::new(&tga, Point::zero());
    image.draw(&mut display).unwrap();
    let mut wolf = entities::Wolf::new();
    wolf.draw(&mut display);
    //assert_eq!(tga2.header.color_map_depth, 2);
    //let image2 = Image::new(&tga2, Point::new(92, 58));
    //display.draw_pixel(item)
    //  image2.draw(&mut display).unwrap();

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown { .. } => {
                    wolf.translate(10, 0);
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    println!("Click event at ({}, {})", point.x, point.y);
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }

            thread::sleep(Duration::from_millis(200));
            //window.update(&display);
        }
    }
}
