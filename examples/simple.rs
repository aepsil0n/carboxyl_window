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

use glium::Surface;
use glium_graphics::{ Glium2d, GliumGraphics, GliumWindow, GlyphCache };
use glutin_window::GlutinWindow;
use window::WindowSettings;
use carboxyl::Cell;
use carboxyl_window::{ StreamingWindow, WindowWrapper };
use shader_version::OpenGL;
use elmesque::{ Form, Renderer };
use elmesque::color::{ Color, rgb, hsl };


#[derive(Clone, Debug)]
struct Model {
    position: (f64, f64),
    color: Color,
    text: String,
}


/// Some trivial application logic
fn app_logic<W: StreamingWindow>(window: &W) -> Cell<Model> {
    lift!(
        |pos, wheel| {
            Model {
                position: pos,
                color: hsl(wheel.1 as f32 / 20.0, 1.0, 0.5),
                text: format!("{:?}", wheel.1),
            }
        },
        &window.cursor(),
        &window.wheel()
    )
}

/// A functional view
fn view(model: Model) -> Form {
    use elmesque::form::{ group, circle, text };
    use elmesque::text::Text;
    let (x, y) = model.position;
    group(vec![
        circle(60.0).filled(model.color),
        text(Text::from_string(model.text)
            .color(rgb(1.0, 1.0, 1.0))),
    ])
    .shift(x as f64, -y as f64)
}


fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::path::Path;

    let glutin_window = Rc::new(RefCell::new(GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new("Title", (1920, 1080))
    )));
    let window = WindowWrapper::new(glutin_window.clone(), 10_000_000);
    let scene = lift!(|s, m| (s, view(m)), &window.size(), &app_logic(&window));
    let glium_window = GliumWindow::new(&glutin_window).unwrap();
    let mut backend_sys = Glium2d::new(OpenGL::_3_2, &glium_window);
    let mut glyph_cache = GlyphCache::new(
        &Path::new("./assets/NotoSans/NotoSans-Regular.ttf"),
        glium_window.clone()
    ).unwrap();

    window.run(|| {
        let ((w, h), form) = scene.sample();
        let form = form.shift(
            -(w as f64 / 2.0),
            h as f64 / 2.0
        );
        let mut target = glium_window.draw();
        {
            let mut backend = GliumGraphics::new(&mut backend_sys, &mut target);
            graphics::clear([0., 0., 0., 1.], &mut backend);
            let mut renderer = Renderer::new(w as f64, h as f64, &mut backend)
                .character_cache(&mut glyph_cache);
            elmesque::form::collage(w as i32, h as i32, vec![form])
                .draw(&mut renderer);
        }
        target.finish();
    });
}
