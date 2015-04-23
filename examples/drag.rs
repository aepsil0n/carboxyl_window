//! Mouse drag & drop demo application
//!
//! This example demonstrates how to build complex event handling logic using
//! Carboxyl's primitives in a practical application. Elmesque is used for
//! visualization.
//!
//! # Controls
//!
//! - SPACE: spawn a square
//! - Left mouse button: drag & drop squares around

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
use input::{ Button, Key, MouseButton };
use carboxyl::{ CellCycle, Cell, Stream };
use carboxyl_window::{ StreamingWindow, WindowWrapper, ButtonState,
                       ButtonEvent };
use shader_version::OpenGL;
use elmesque::{ Form, Renderer };


/// One of little rectangles making up the application model
#[derive(Copy, Clone, Debug)]
struct Rect(f64, f64);

impl Rect {
    /// Does the rectangle contain a given position?
    pub fn contains(&self, (x, y): (f64, f64)) -> bool {
        let Rect(x0, y0) = *self;
           (x > x0 - 50.0) && (x < x0 + 50.0)
        && (y > y0 - 50.0) && (y < y0 + 50.0)
    }
}

/// A drag & drop event
#[derive(Clone, Debug)]
enum Pick {
    Drag((f64, f64)),
    Drop,
}

/// Reactive drag & drop logic
fn drag_n_drop(position: &Cell<(f64, f64)>, clicks: &Stream<ButtonState>)
    -> Stream<Pick>
{
    position.snapshot(&clicks)
        .map(|(pos, state)| match state {
            ButtonState::Pressed => Pick::Drag(pos),
            ButtonState::Released => Pick::Drop,
        })
}

/// Filter left clicks from a button event
fn left_clicks(event: ButtonEvent) -> Option<ButtonState> {
    match event {
        ButtonEvent { button: Button::Mouse(MouseButton::Left), state }
            => Some(state),
        _   => None,
    }
}

/// Has space been pressed?
fn space_pressed(event: &ButtonEvent) -> bool {
    event.button == Button::Keyboard(Key::Space) &&
    event.state == ButtonState::Pressed
}

/// Find the index of the first in a list of rects containing a point
fn find_index(pos: (f64, f64), rects: &Vec<Rect>) -> Option<usize> {
    rects.iter().enumerate()
        .filter_map(|(k, r)|
            if r.contains(pos) { Some(k) }
            else { None }
        ).next()
}

/// How the rects behave while dragging
fn drag_cell(pos: (f64, f64), start: Vec<Rect>, cursor: &Cell<(f64, f64)>) -> Cell<Vec<Rect>> {
    match find_index(pos, &start) {
        Some(idx) => lift!(move |(x, y)| {
            let mut start = start.clone();
            let Rect(x0, y0) = start[idx];
            start[idx] = Rect(x0 + (x - pos.0), y0 + (y - pos.1));
            start
        }, cursor),
        None => Stream::never().hold(start),
    }
}

/// Stream of rects spawned
fn spawned_rects(cursor: &Cell<(f64, f64)>, buttons: &Stream<ButtonEvent>)
    -> Stream<Rect>
{
    cursor.snapshot(&buttons.filter(space_pressed))
        .map(|(pos, _)| Rect(pos.0, pos.1))
}

/// Overall application logic
fn app_logic<W: StreamingWindow>(window: &W) -> Cell<Vec<Rect>> {
    // Bind relevant window attributes
    let buttons = window.buttons();
    let cursor = window.cursor();

    // The drag & drop event stream
    let picks = drag_n_drop(
        &lift!(|(x, y)| (x as f64, y as f64), &cursor),
        &buttons.filter_map(left_clicks)
    );

    // Forward declaration of the output cell
    let rects: CellCycle<Vec<Rect>> = CellCycle::new(vec![]);

    // Behaviour initiated by drag & drop events
    let drag_drop_cell = {
        let cursor = cursor.clone();
        rects.snapshot(&picks)
            .map(move |(rects, pick)| match pick {
                Pick::Drag(pos) => drag_cell(pos, rects, &cursor),
                Pick::Drop => Stream::never().hold(rects),
            })
    };

    // Behaviour initiated by spawn events
    let spawn_cell = rects
        .snapshot(&spawned_rects(&cursor, &buttons))
        .map(|(mut rects, r)| {
            rects.push(r);
            Stream::never().hold(rects)
        });

    // Now define the output cell
    rects.define(
        // Merge the cell streams
        drag_drop_cell.merge(&spawn_cell)
        // Hold onto an initial cell with no rects
        .hold(Stream::never().hold(vec![]))
        // And switch to flatten this to a cell of rects
        .switch()
    )
}

/// Functional view of a vector of rects
fn view(rects: &Vec<Rect>) -> Form {
    use elmesque::color::rgba;
    use elmesque::form::{ group, rect };
    group(
        rects.iter()
        .map(|&Rect(x, y)|
            rect(100.0, 100.0)
            .filled(rgba(1.0, 0.3, 0.0, 0.7))
            .shift(x as f64, -y as f64)
        )
        .collect()
    )
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
    let model = app_logic(&window);
    let scene = lift!(|s, r| (s, view(&r)), &window.size(), &model);
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
            graphics::clear([1.0; 4], &mut backend);
            let mut renderer = Renderer::new(w as f64, h as f64, &mut backend)
                .character_cache(&mut glyph_cache);
            elmesque::form::collage(w as i32, h as i32, vec![form])
                .draw(&mut renderer);
        }
        target.finish();
    });
}
