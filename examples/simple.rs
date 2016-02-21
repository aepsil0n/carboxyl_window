//! Simple interactive application
//!
//! This example is very simple. You can move around a circle that follows the
//! mouse position. Using the mouse cursor you can change the color of the
//! circle and a little number displayed on its center.
//!
//! The event handling logic here is trivial, since the output is a very simple
//! function of input events. It is intended to demonstrate how you can set up
//! an application using carboxyl, carboxyl_window and elmesque.

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
use elmesque::Element;
use elmesque::color::{Color, hsl};

mod runners;


#[derive(Clone, Debug)]
struct Model {
    position: (f64, f64),
    color: Color,
    text: String,
}


fn draw_colorful_circle(position: (f64, f64), wheel: (f64, f64)) -> Model {
    Model {
        position: position,
        color: hsl(wheel.1 as f32 / 20.0, 1.0, 0.5),
        text: format!("{:?}", wheel.1),
    }
}

fn app_logic<W: StreamingWindow>(window: &W) -> Signal<Model> {
    window.context().map(
        |ctx| draw_colorful_circle(ctx.cursor.position, ctx.cursor.wheel))
}

fn view((width, height): (u32, u32), model: Model) -> Element {
    use elmesque::form::{collage, group, circle, text};
    use elmesque::text::Text;
    use elmesque::color::{rgb, black};

    let (x, y) = model.position;
    collage(width as i32,
            height as i32,
            vec![group(vec![
            circle(60.0).filled(model.color),
            text(Text::from_string(model.text)
                .color(rgb(1.0, 1.0, 1.0))),
        ])
                     .shift(x as f64, -y as f64)
                     .shift(-(width as f64 / 2.0), height as f64 / 2.0)])
        .clear(black())
}

fn main() {
    runners::run_glutin(WindowSettings::new("carboxyl_window :: example/simple.rs", (640, 480)),
        |window| lift!(|ctx, model| view(ctx.window.size, model),
            &window.context(), &app_logic(window)));
}
