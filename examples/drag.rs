extern crate graphics;
extern crate glium;
extern crate glutin;
extern crate glium_graphics;
extern crate image;
extern crate input;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;


fn main() {
    use glium::{DisplayBuild, Surface};
    use glium_graphics::{GliumBackendSystem, GliumSurfaceBackEnd};
    use graphics::Context;
    use input::{Button, Key, MouseButton};
    use carboxyl::CellCycle;
    use carboxyl_window::{GliumWindow, Window, ButtonState, ButtonEvent};

    let display = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title(format!("Image test"))
        .build_glium().unwrap();

    let window = GliumWindow::new(display.clone(), 60);
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
    let rects = rects.define(new_rects);

    let _render = lift!(|s, r| (s, r), &window.size(), &rects)
        .snapshot(&window.ticks())
        .map(move |(((w, h), rects), _dt)| {
            let mut target = display.draw();
            {
                let mut backend_sys = GliumBackendSystem::new(&display);
                let mut backend = GliumSurfaceBackEnd::new(&mut backend_sys, &mut target);
                let context = Context::abs(w as f64, h as f64);
                graphics::clear([1.0; 4], &mut backend);
                /*if let Some((_, drag)) = drag {
                    graphics::Rectangle::new([0.0, 0.8, 0.0, 0.7])
                    .draw(
                        [(drag.0 - 50) as f64, (drag.1 - 50) as f64, 100.0, 100.0],
                        &context, &mut backend
                    );
                }*/
                for rect in rects {
                    graphics::Rectangle::new([1.0, 0.3, 0.0, 0.7])
                    .draw(
                        [(rect.0 - 50) as f64, (rect.1 - 50) as f64, 100.0, 100.0],
                        &context, &mut backend
                    )
                }
            }
            target.finish();
        });

    window.start();
}

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
