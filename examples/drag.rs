extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;
extern crate input;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;


use glium::{ DisplayBuild, Surface };
use glium_graphics::{ Glium2d, GliumGraphics, OpenGL };
use input::{ Button, Key, MouseButton };
use carboxyl::{ CellCycle, Cell };
use carboxyl_window::{ GliumWindow, Window, ButtonState, ButtonEvent };


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


fn app_logic(window: &GliumWindow) -> Cell<Vec<Rect>> {
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
                => Some(RectEvent::Spawn(Rect(pos.0, pos.1))),
            ButtonEvent { button: Button::Mouse(MouseButton::Left), state: ButtonState::Pressed }
                => rects.iter()
                    .enumerate()
                    .filter_map(|(k, r)|
                        if r.contains(pos) { Some(RectEvent::Drag(k, pos)) }
                        else { None })
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
                        rects[idx].0 + (mouse.0 - pos.0),
                        rects[idx].1 + (mouse.1 - pos.1)
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
    let display = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title(format!("Image test"))
        .build_glium().unwrap();

    let window = GliumWindow::new(display, 60);
    let rects = app_logic(&window);
    let scene = lift!(|s, r| (s, r), &window.size(), &rects);

    window.run(|display| {
        let ((w, h), rects) = scene.sample();
        let mut target = display.draw();
        {
            let mut backend_sys = Glium2d::new(OpenGL::_3_2, &display);
            let mut backend = GliumGraphics::new(&mut backend_sys, &mut target);
            let transform = graphics::abs_transform(w as f64, h as f64);
            graphics::clear([1.0; 4], &mut backend);
            for rect in rects {
                graphics::Rectangle::new([1.0, 0.3, 0.0, 0.7])
                .draw(
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
