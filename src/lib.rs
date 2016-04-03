#[macro_use(lift)]
extern crate carboxyl;
extern crate clock_ticks;
extern crate glutin;

pub use source_win::SourceWindow;

mod source_win;
mod updates;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowProperties {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub focus: bool
}

impl WindowProperties {
    pub fn new(position: (i32, i32), size: (u32, u32), focus: bool)
        -> WindowProperties
    {
        WindowProperties {
            position: position,
            size: size,
            focus: focus
        }
    }
}

impl Default for WindowProperties {
    fn default() -> Self {
        WindowProperties {
            position: (0, 0),
            size: (0, 0),
            focus: true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Cursor {
    pub position: (f64, f64),
    pub wheel: (f64, f64)
}

impl Cursor {
    pub fn new(position: (f64, f64), wheel: (f64, f64)) -> Cursor {
        Cursor { position: position, wheel: wheel }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Context {
    pub window: WindowProperties,
    pub cursor: Cursor
}

impl Context {
    pub fn new(window: WindowProperties, cursor: Cursor) -> Context {
        Context { window: window, cursor: cursor }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Press(()),
    Release(()),
    Text(String)
}
