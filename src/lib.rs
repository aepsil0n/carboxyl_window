#[macro_use(lift)]
extern crate carboxyl;
extern crate clock_ticks;
extern crate input;
extern crate window;

pub use button::*;
pub use wrapper::WindowWrapper;

use carboxyl::{ Stream, Signal };

mod button;
mod wrapper;


/// An abstraction of window I/O.
pub trait StreamingWindow {
    /// Position of the window
    fn position(&self) -> Signal<(i32, i32)>;

    /// Size of the window
    fn size(&self) -> Signal<(u32, u32)>;

    /// Stream of input events.
    fn buttons(&self) -> Stream<ButtonEvent>;

    /// A stream of characters received by the window
    fn text(&self) -> Stream<String>;

    /// Position of the mouse cursor
    fn cursor(&self) -> Signal<(f64, f64)>;

    /// Position of the mouse wheel
    fn wheel(&self) -> Signal<(f64, f64)>;

    /// Window focus
    fn focus(&self) -> Signal<bool>;
}
