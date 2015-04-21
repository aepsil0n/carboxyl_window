extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate shader_version;
extern crate image;
extern crate input;
extern crate window;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;


use glium::{ DisplayBuild, Surface };
use glium_graphics::{ Glium2d, GliumGraphics, GliumWindow };
use glutin_window::{ GlutinWindow };
use window::WindowSettings;
use input::{ Button, Key, MouseButton };
use carboxyl::{ CellCycle, Cell };
use carboxyl_window::{ StreamingWindow, WindowWrapper, ButtonState,
                       ButtonEvent };
use shader_version::OpenGL;


#[derive(Clone, Debug)]
enum RectEvent {
    Spawn(Rect),
    Drag(usize, (i32, i32)),
    Drop,
}


#[derive(Clone, Debug)]
struct Rect(i32, i32);

impl Rect {
    pub fn contains(&self, pos: (i32, i32)) -> bool {
           (pos.0 > self.0 - 50)
        && (pos.0 < self.0 + 50)
        && (pos.1 > self.1 - 50)
        && (pos.1 < self.1 + 50)
    }
}


fn app_logic<W: StreamingWindow>(window: &W) -> Cell<Vec<Rect>> {
    let buttons = window.buttons();
    let cursor = window.cursor();

    let rects = CellCycle::<Vec<Rect>>::new(vec![Rect(100, 100)]);

    let events = lift!(|c, r| (c, r), &cursor, &rects)
        .snapshot(&buttons)
        .filter_map(|((pos, rects), button)| match button {
            ButtonEvent {
                button: Button::Keyboard(Key::Space),
                state: ButtonState::Pressed,
            }
                => Some(RectEvent::Spawn(Rect(pos.0 as i32, pos.1 as i32))),
            ButtonEvent { button: Button::Mouse(MouseButton::Left), state: ButtonState::Pressed }
                => rects.iter()
                    .enumerate()
                    .filter_map(|(k, r)| {
                        let pos = (pos.0 as i32, pos.1 as i32);
                        if r.contains(pos) { Some(RectEvent::Drag(k, pos)) }
                        else { None }
                    })
                    .next(),
            ButtonEvent { button: Button::Mouse(MouseButton::Left), state: ButtonState::Released }
                => Some(RectEvent::Drop),
            _   => None,
        });

    let spawned = rects.snapshot(&events)
        .map(|(mut rects, ev)| match ev {
            RectEvent::Spawn(r) => { rects.push(r); rects },
            _ => rects,
        })
        .hold(vec![Rect(300, 500)]);

    let new_rects = events.filter_map({
        let spawned = spawned.clone();
        move |ev| match ev {
            RectEvent::Drag(idx, pos) => {
            Some(lift!(
                move |mut rects, mouse| {
                    rects[idx] = Rect(
                        rects[idx].0 + (mouse.0 as i32 - pos.0),
                        rects[idx].1 + (mouse.1 as i32 - pos.1)
                    );
                    rects
                },
                &spawned, &cursor
            ))},
            RectEvent::Drop => Some(spawned.clone()),
            _ => None,
        }})
        .hold(spawned.clone())
        .switch();
    rects.define(new_rects)
}


fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;

    let glutin_window = Rc::new(RefCell::new(GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new("Title", (1920, 1080))
    )));
    let window = WindowWrapper::new(glutin_window.clone(), 60);
    let rects = app_logic(&window);
    let scene = lift!(|s, r| (s, r), &window.size(), &rects);
    let glium_window = GliumWindow::new(&glutin_window).unwrap();
    let mut backend_sys = Glium2d::new(OpenGL::_3_2, &glium_window);

    window.run(|| {
        let ((w, h), rects) = scene.sample();
        let mut target = glium_window.draw();
        {
            let mut backend = GliumGraphics::new(&mut backend_sys, &mut target);
            let transform = graphics::math::abs_transform(w as f64, h as f64);
            graphics::clear([1.0; 4], &mut backend);
            for rect in rects {
                graphics::Rectangle::new([1.0, 0.3, 0.0, 0.7]).draw(
                    [(rect.0 - 50) as f64, (rect.1 - 50) as f64, 100.0, 100.0],
                    graphics::default_draw_state(),
                    transform,
                    &mut backend
                )
            }
        }
        target.finish();
    });
}
