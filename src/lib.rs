#[macro_use(lift)]
extern crate carboxyl;
extern crate clock_ticks;
extern crate input;
extern crate window;

use carboxyl::{Stream, Signal};

pub use source_win::SourceWindow;

pub mod button;
mod source_win;
mod borrowing;


pub trait Driver {
    fn context(&self) -> Signal<Context>;
    fn events(&self) -> Stream<Event>;
}


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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cursor {
    pub position: (f64, f64),
    pub wheel: (f64, f64)
}

impl Cursor {
    pub fn new(position: (f64, f64), wheel: (f64, f64)) -> Cursor {
        Cursor { position: position, wheel: wheel }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Press(input::Button),
    Release(input::Button),
    Text(String)
}

/// An abstraction of a window's event generation facilities.
pub trait RunnableWindow {
    /// Run the window, calling a function every frame
    fn run_with<F: FnMut()>(&mut self, fps: f64, render: F);

    /// Run the window generating events
    fn run(&mut self, fps: f64) {
        self.run_with(fps, || ());
    }
}
