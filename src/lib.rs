#[macro_use(lift)]
extern crate carboxyl;
extern crate clock_ticks;
extern crate glutin;
extern crate glium;
extern crate input;
extern crate glutin_window;
extern crate graphics;
extern crate glium_graphics;

pub use button::*;
pub use glium_impl::GliumWindow;
use carboxyl::{ Stream, Cell };

mod glium_impl;
mod button;


/// An abstraction of window I/O.
pub trait Window {
    /// Position of the window
    fn position(&self) -> Cell<(i32, i32)>;

    /// Size of the window
    fn size(&self) -> Cell<(u32, u32)>;

    /// Stream of input events.
    fn buttons(&self) -> Stream<ButtonEvent>;

    /// A stream of characters received by the window
    fn characters(&self) -> Stream<char>;

    /// Position of the mouse cursor
    fn cursor(&self) -> Cell<(i32, i32)>;

    /// Position of the mouse wheel
    fn wheel(&self) -> Cell<i32>;

    /// Window focus
    fn focus(&self) -> Cell<bool>;
}
