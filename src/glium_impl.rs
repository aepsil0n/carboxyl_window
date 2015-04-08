use std::thread;
use std::sync::mpsc;
use glium::{ Display };
use glutin::{Event, ElementState};
use clock_ticks::precise_time_ns;
use carboxyl::{Cell, Sink, Stream};
use input::Button;
use glutin_window::{map_key, map_mouse};

use button::{ButtonEvent, ButtonState};
use ::Window;


/// A wrapper for all event sinks required for implementation
struct EventSinks {
    window_position: Sink<(i32, i32)>,
    window_size: Sink<(u32, u32)>,
    button: Sink<ButtonEvent>,
    mouse_motion: Sink<(i32, i32)>,
    mouse_wheel: Sink<i32>,
    focus: Sink<bool>,
    character: Sink<char>,
}

impl EventSinks {
    fn dispatch(&self, event: Event) {
        fn to_button_state(state: ElementState) -> ButtonState {
            match state {
                ElementState::Pressed => ButtonState::Pressed,
                ElementState::Released => ButtonState::Released,
            }
        }

        match event {
            Event::KeyboardInput(state, _, Some(vkey)) =>
                self.button.send(ButtonEvent {
                    button: Button::Keyboard(map_key(vkey)),
                    state: to_button_state(state),
                }),
            Event::MouseInput(state, button) =>
                self.button.send(ButtonEvent {
                    button: Button::Mouse(map_mouse(button)),
                    state: to_button_state(state),
                }),
            Event::MouseWheel(a) => self.mouse_wheel.send(a),
            Event::MouseMoved(a) => self.mouse_motion.send(a),
            Event::Focused(a) => self.focus.send(a),
            Event::Resized(w, h) => self.window_size.send((w, h)),
            Event::Moved(x, y) => self.window_position.send((x, y)),
            Event::ReceivedCharacter(c) => self.character.send(c),
            _ => (),
        }
    }
}


/// Glium implementation of an application loop.
pub struct GliumWindow {
    display: Display,
    tick_length: u64,
    sinks: EventSinks,
}

impl GliumWindow {
    /// Create a new Glium loop.
    ///
    /// # Parameters
    ///
    /// `tick_length` is the minimum duration of a tick in nanoseconds.
    pub fn new(display: Display, tick_length: u64) -> GliumWindow {
        GliumWindow {
            display: display,
            tick_length: tick_length,
            sinks: EventSinks {
                button: Sink::new(),
                mouse_motion: Sink::new(),
                mouse_wheel: Sink::new(),
                focus: Sink::new(),
                window_position: Sink::new(),
                window_size: Sink::new(),
                character: Sink::new(),
            }
        }
    }

    pub fn run<F: FnMut(&Display)>(&self, mut render: F) {
        let (tx, rx) = mpsc::channel();
        let sinks = &self.sinks;
        let event_thread = thread::scoped(move || {
            for event in rx.iter() {
                if let Event::Closed = event { break }
                sinks.dispatch(event);
            }
        });
        let mut time = precise_time_ns();
        let mut next_tick = time;
        while !self.display.is_closed() {
            time = precise_time_ns();
            if time >= next_tick {
                let diff = time - next_tick;
                let delta = diff - diff % self.tick_length;
                next_tick += delta;
                for ev in self.display.poll_events() { tx.send(ev); }
                render(&self.display);
            }
            else {
                thread::sleep_ms((next_tick - time) as u32);
            }
        }
    }
}

impl Window for GliumWindow {
    fn position(&self) -> Cell<(i32, i32)> {
        self.sinks.window_position.stream().hold((0, 0))
    }

    fn size(&self) -> Cell<(u32, u32)> {
        self.sinks.window_size.stream().hold((0, 0))
    }

    fn buttons(&self) -> Stream<ButtonEvent> {
        self.sinks.button.stream()
    }

    fn characters(&self) -> Stream<char> {
        self.sinks.character.stream()
    }

    fn cursor(&self) -> Cell<(i32, i32)> {
        self.sinks.mouse_motion.stream().hold((0, 0))
    }

    fn wheel(&self) -> Cell<i32> {
        self.sinks.mouse_wheel.stream().hold(0)
    }

    fn focus(&self) -> Cell<bool> {
        self.sinks.focus.stream().hold(true)
    }
}
