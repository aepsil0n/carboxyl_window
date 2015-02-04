use carboxyl::Stream;
use button::ButtonEvent;


/// An abstraction of an I/O loop.
pub trait ApplicationLoop {
    /// Stream of discrete time intervals (ticks).
    fn ticks(&self) -> Stream<u64>;

    /// Stream of input events.
    ///
    /// FIXME: need to ship our own event type here and make it consistent with
    /// the paradigm.
    fn buttons(&self) -> Stream<ButtonEvent>;

    /// Start the application logic.
    fn start(&self);
}
