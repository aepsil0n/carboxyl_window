use std::cell::RefCell;
use std::rc::Rc;
use glium::Surface;
use glium_graphics::{Glium2d, GliumGraphics, GliumWindow, GlyphCache};
use glutin_window::GlutinWindow;
use carboxyl_window::{RunnableWindow, StreamingWindow, SourceWindow};
use carboxyl::Signal;
use elmesque::{Element, Renderer};
use shader_version::OpenGL;
use window::WindowSettings;
use graphics::context::Context;

pub fn run_glutin<F>(settings: WindowSettings, app: F)
    where F: FnOnce(&SourceWindow<Rc<RefCell<GlutinWindow>>>) -> Signal<Element>
{
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::path::Path;

    const GLVERSION: OpenGL = OpenGL::V2_1;
    let glutin_window = Rc::new(RefCell::new(GlutinWindow::new(settings).ok().unwrap()));
    let mut window = SourceWindow::new(glutin_window.clone());
    let scene = lift!(|s, m| (s, m), &window.size(), &app(&window));
    let glium_window = GliumWindow::new(&glutin_window).ok().unwrap();
    let mut backend_sys = Glium2d::new(GLVERSION, &glium_window);
    let mut glyph_cache = GlyphCache::new(&Path::new("./assets/NotoSans/NotoSans-Regular.ttf"),
                                          glium_window.clone())
                              .unwrap();

    window.run_with(120.0, || {
        let ((w, h), element) = scene.sample();
        let mut target = glium_window.draw();
        {
            let mut backend = GliumGraphics::new(&mut backend_sys, &mut target);
            let mut renderer = Renderer::new(Context::new_abs(w as f64, h as f64), &mut backend)
                                   .character_cache(&mut glyph_cache);
            element.draw(&mut renderer);
        }
        target.finish().unwrap();
    });
}
