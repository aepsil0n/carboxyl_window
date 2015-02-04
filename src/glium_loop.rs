use std::time::duration::Duration;
use std::old_io::timer::sleep;
use glium::Display;
use glutin::{Event, ElementState};
use clock_ticks::precise_time_ns;
use carboxyl::{Sink, Stream};

use button::{Button, ButtonEvent, ButtonState};
use traits::ApplicationLoop;


/// Glium implementation of an application loop.
pub struct GliumLoop {
    display: Display,
    tick_length: u64,
    tick_sink: Sink<u64>,
    button_sink: Sink<ButtonEvent>,
}

impl GliumLoop {
    /// Create a new Glium loop.
    ///
    /// # Parameters
    ///
    /// `tick_length` is the minimum duration of a tick in nanoseconds.
    pub fn new(display: Display, tick_length: u64) -> GliumLoop {
        GliumLoop {
            display: display,
            tick_length: tick_length,
            tick_sink: Sink::new(),
            button_sink: Sink::new(),
        }
    }

    fn dispatch(&self, event: Event) {
        match event {
            Event::KeyboardInput(state, _, Some(virt)) =>
                self.button_sink.send(ButtonEvent {
                    button: Button::Glutin(virt),
                    state: match state {
                        ElementState::Pressed => ButtonState::Pressed,
                        ElementState::Released => ButtonState::Released,
                    },
                }),
            // TODO: handle all events
            _ => (),
        }
    }
}

impl ApplicationLoop for GliumLoop {
    fn ticks(&self) -> Stream<u64> {
        self.tick_sink.stream()
    }

    fn buttons(&self) -> Stream<ButtonEvent> {
        self.button_sink.stream()
    }

    fn start(&self) {
        let mut time = precise_time_ns();
        let mut next_tick = time;
        loop {
            time = precise_time_ns();
            if time >= next_tick {
                let diff = time - next_tick;
                let delta = diff - diff % self.tick_length;
                next_tick += delta;
                for ev in self.display.poll_events() {
                    self.dispatch(ev);
                }
                self.tick_sink.send(delta);
            }
            else {
                sleep(Duration::nanoseconds(next_tick as i64 - time as i64));
            }
        }
    }
}
