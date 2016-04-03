use std::thread;
use std::time::Duration;
use clock_ticks::precise_time_ns;
use carboxyl::{Signal, Sink, Stream};
use glutin;
use ::{Event, Context};
use updates::Update;


fn state_update(event: glutin::Event) -> Option<Update> {
    use updates::WindowUpdate::*;
    use updates::CursorUpdate::{self, WheelDelta};
    use glutin::Event::*;
    use glutin::MouseScrollDelta;

    Some(match event {
        Resized(width, height) =>
            Update::Window(Resize(width, height)),
        Moved(x, y) =>
            Update::Window(MoveTo(x, y)),
        MouseMoved((x, y)) =>
            Update::Cursor(CursorUpdate::MoveTo(x as f64, y as f64)),
        MouseWheel(MouseScrollDelta::PixelDelta(x, y)) =>
            Update::Cursor(WheelDelta(x as f64, y as f64)),
        Focused(state) =>
            Update::Window(Focus(state)),
        _ => return None
    })
}


pub struct WindowDriver {
    window: glutin::Window,
    event_sink: Sink<Event>,
    update_sink: Sink<Update>
}

impl WindowDriver {
    pub fn new(window: glutin::Window) -> WindowDriver {
        WindowDriver {
            window: window,
            event_sink: Sink::new(),
            update_sink: Sink::new()
        }
    }

    pub fn run_with<F: FnMut(&glutin::Window)>(&mut self, fps: f64, mut render: F) {
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
                for event in self.window.poll_events() {
                    if let glutin::Event::Closed = event {
                        should_close = true;
                    }
                    self.dispatch(event)
                }
                render(&self.window);
            } else {
                thread::sleep(Duration::from_millis((next_tick - time) as u64));
            }
        }
    }

    fn dispatch(&self, event: glutin::Event) {
        if let Some(update) = state_update(event) {
            self.update_sink.send(update);
        }
    }

    pub fn context(&self) -> Signal<Context> {
        self.update_sink.stream()
            .fold(Context::default(), |old, update| update.apply(old))
    }

    pub fn events(&self) -> Stream<Event> {
        self.event_sink.stream()
    }
}
