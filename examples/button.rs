//! Simple interactive application

extern crate elmesque;
extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate input;
extern crate window;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;

use window::WindowSettings;
use carboxyl::Signal;
use carboxyl_window::StreamingWindow;
use elmesque::{Element, Form};
use elmesque::text::Text;
use elmesque::color::{rgb, black, white};
use elmesque::form::{collage, group, circle, text};

mod runners;


fn app_logic<W: StreamingWindow>(window: &W) -> Signal<(f64, f64)> {
    lift!(|pos| pos, &window.cursor())
}

fn hello() -> Form {
    text(Text::from_string("Hello!".to_string())
        .color(white())
        .height(50.))
}

fn view((width, height): (u32, u32), model: (f64, f64)) -> Element {

    let (x, y) = model;
    collage(width as i32,
            height as i32,
            vec![hello()])
        .clear(black())
}

fn main() {
    runners::run_glutin(WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480)),
                        |window| lift!(view, &window.size(), &app_logic(window)));
}
