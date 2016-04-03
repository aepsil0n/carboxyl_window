use std::thread;
use std::time::Duration;
use clock_ticks::precise_time_ns;
use carboxyl::{Signal, Sink, Stream};
use glutin;
use ::{Event, Context};
use updates::Update;


/// A reactive window implementation generic over the event source.
pub struct SourceWindow {
    window: glutin::Window,
    event_sink: Sink<Event>,
    update_sink: Sink<Update>
}

impl SourceWindow {
    pub fn new(window: glutin::Window) -> SourceWindow {
        SourceWindow {
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
        use updates::WindowUpdate::*;
        use updates::CursorUpdate::{self, WheelDelta};
        use glutin::Event::*;
        use glutin::MouseScrollDelta;

        match event {
            Resized(width, height) =>
                self.update_sink.send(Update::Window(Resize(width, height))),
            Moved(x, y) =>
                self.update_sink.send(Update::Window(MoveTo(x, y))),
            MouseMoved((x, y)) =>
                self.update_sink.send(Update::Cursor(
                        CursorUpdate::MoveTo(x as f64, y as f64))),
            MouseWheel(MouseScrollDelta::PixelDelta(x, y)) =>
                self.update_sink.send(Update::Cursor(WheelDelta(x as f64, y as f64))),
            Focused(state) =>
                self.update_sink.send(Update::Window(Focus(state))),
            _ => ()
        }
    }

    pub fn context(&self) -> Signal<Context> {
        self.update_sink.stream()
            .fold(
                Default::default(),
                |old, update| update.apply(old)
            )
    }

    pub fn events(&self) -> Stream<Event> {
        self.event_sink.stream()
    }
}
