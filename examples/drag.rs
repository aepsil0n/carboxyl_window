#![feature(core)]

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
    use glium_graphics::GliumSurfaceBackEnd;
    use graphics::Context;
    use input::{Button, Key, MouseButton};
    //use carboxyl::{Cell, Sink};
    use carboxyl_window::{GliumWindow, Window, ButtonState, ButtonEvent};

    let display = glutin::WindowBuilder::new()
        .with_dimensions(300, 300)
        .with_title(format!("Image test"))
        .build_glium().unwrap();

    let window = GliumWindow::new(display.clone(), 60);
    let buttons = window.buttons();
    let cursor = window.cursor();

    let spawn = cursor.snapshot(
        &buttons.map(|button| match button {
            ButtonEvent {
                button: Button::Keyboard(Key::Space),
                state: ButtonState::Pressed,
            } => Some(()),
            _ => None,
        }).filter()
    ).map(|(pos, _)| Rect(pos.0, pos.1));

    let clicks = buttons.map(|button| match button {
        ButtonEvent { button: Button::Mouse(MouseButton::Left), state }
          => Some(state),
        _ => None,
    }).filter();

    let spawned_rects = spawn.accumulate(vec![],
        |(mut last, new)| { last.push(new); last });

    let drag = lift!(|a, b| (a, b), &spawned_rects, &cursor)
        .snapshot(&clicks)
        .map(|((rects, pos), state)| match state {
            ButtonState::Pressed => rects.iter()
                .enumerate()
                .filter_map(|(k, r)| if r.contains(pos) { Some(k) } else { None })
                .next(),
            ButtonState::Released => None,
        })
        .hold(None);

    let drag_rect = lift!(|rects, idx| match idx {
        Some(idx) => Some(Rect(rects[idx].0 + 200, rects[idx].1)),
        None => None,
    }, &spawned_rects, &drag);

    let _render = lift!(|s, r, d| (s, r, d), &window.size(), &spawned_rects, &drag_rect)
        .snapshot(&window.ticks())
        .map(move |(((w, h), rects, drag), _dt)| {
            let mut target = display.draw();
            {
                let mut backend = GliumSurfaceBackEnd::new(display.clone(), &mut target);
                let context = Context::abs(w as f64, h as f64);
                graphics::clear([1.0; 4], &mut backend);
                if let Some(drag) = drag {
                    graphics::Rectangle::new([0.0, 0.8, 0.0, 0.6])
                    .draw(
                        [(drag.0 - 50) as f64, (drag.1 - 50) as f64, 100.0, 100.0],
                        &context, &mut backend
                    );
                }
                for rect in rects {
                    graphics::Rectangle::new([1.0, 0.5, 0.0, 0.6])
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
struct Rect(i32, i32);

impl Rect {
    pub fn contains(&self, pos: (i32, i32)) -> bool {
           (pos.0 > self.0 - 50)
        && (pos.0 < self.0 + 50)
        && (pos.1 > self.1 - 50)
        && (pos.1 < self.1 + 50)
    }
}
