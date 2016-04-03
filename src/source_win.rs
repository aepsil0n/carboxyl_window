use std::thread;
use std::time::Duration;
use clock_ticks::precise_time_ns;
use carboxyl::{Signal, Sink, Stream};
use glutin;
use ::{RunnableWindow, Event, Context, WindowProperties, Cursor, StreamingWindow};


/// A wrapper for all event sinks required for implementation
struct EventSinks {
    event: Sink<Event>,
    window_position: Sink<(i32, i32)>,
    window_size: Sink<(u32, u32)>,
    mouse_motion: Sink<(f64, f64)>,
    mouse_wheel: Sink<(f64, f64)>,
    focus: Sink<bool>,
}

impl EventSinks {
    fn dispatch(&self, event: glutin::Event) {
        // TODO
    }
}


/// A reactive window implementation generic over the event source.
pub struct SourceWindow {
    source: glutin::Window,
    sinks: EventSinks
}

impl SourceWindow {
    /// Create a new Glium loop.
    ///
    /// # Parameters
    ///
    /// `tick_length` is the minimum duration of a tick in nanoseconds.
    pub fn new(window: glutin::Window) -> SourceWindow {
        SourceWindow {
            source: window,
            sinks: EventSinks {
                event: Sink::new(),
                mouse_motion: Sink::new(),
                mouse_wheel: Sink::new(),
                focus: Sink::new(),
                window_position: Sink::new(),
                window_size: Sink::new(),
            }
        }
    }
}

impl RunnableWindow for SourceWindow
{
    fn run_with<F: FnMut()>(&mut self, fps: f64, mut render: F) {
        assert!(fps > 0.0);
        let tick_length = (1e9 / fps) as u64;
        let mut time = precise_time_ns();
        let mut next_tick = time;
        let mut should_close = false;
        while !should_close {
            time = precise_time_ns();
            if time >= next_tick {
                let diff = time - next_tick;
                let delta = diff - diff % tick_length;
                next_tick += delta;
                for event in self.source.poll_events() {
                    if let glutin::Event::Closed = event {
                        should_close = true;
                    }
                    let _ = self.sinks.dispatch(event);
                }
                render();
            } else {
                thread::sleep(Duration::from_millis((next_tick - time) as u64));
            }
        }
    }
}

impl StreamingWindow for SourceWindow {
    fn context(&self) -> Signal<Context> {
        let window = lift!(WindowProperties::new,
            &self.position(), &self.size(), &self.focus());
        let cursor = lift!(Cursor::new, &self.cursor(), &self.wheel());
        lift!(Context::new, &window, &cursor)
    }

    fn events(&self) -> Stream<Event> {
        self.sinks.event.stream()
    }
}

impl SourceWindow {
    fn position(&self) -> Signal<(i32, i32)> {
        self.sinks.window_position.stream().hold((0, 0))
    }

    fn size(&self) -> Signal<(u32, u32)> {
        self.sinks.window_size.stream().hold((0, 0))
    }

    fn cursor(&self) -> Signal<(f64, f64)> {
        self.sinks.mouse_motion.stream().hold((0.0, 0.0))
    }

    fn wheel(&self) -> Signal<(f64, f64)> {
        self.sinks
            .mouse_wheel
            .stream()
            .fold((0.0, 0.0), |(x, y), (dx, dy)| (x + dx, y + dy))
    }

    fn focus(&self) -> Signal<bool> {
        self.sinks.focus.stream().hold(true)
    }
}
