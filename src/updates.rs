use ::{Context, WindowProperties, Cursor};

#[derive(Clone)]
pub enum CursorUpdate {
    MoveTo(f64, f64),
    WheelDelta(f64, f64)
}

impl CursorUpdate {
    pub fn apply(self, current: Cursor) -> Cursor {
        use self::CursorUpdate::*;
        match self {
            MoveTo(x, y) => Cursor { position: (x, y), .. current },
            WheelDelta(dx, dy) =>
                Cursor {
                    wheel: (current.wheel.0 + dx, current.wheel.1 + dy),
                    .. current
                }
        }
    }
}

#[derive(Clone)]
pub enum WindowUpdate {
    Resize(u32, u32),
    MoveTo(i32, i32),
    Focus(bool)
}

impl WindowUpdate {
    pub fn apply(self, current: WindowProperties) -> WindowProperties {
        use self::WindowUpdate::*;
        match self {
            Resize(width, height) =>
                WindowProperties { size: (width, height), .. current },
            MoveTo(x, y) =>
                WindowProperties { position: (x, y), .. current },
            Focus(state) =>
                WindowProperties { focus: state, .. current }
        }
    }
}

#[derive(Clone)]
pub enum Update {
    Window(WindowUpdate),
    Cursor(CursorUpdate)
}

impl Update {
    pub fn apply(self, current: Context) -> Context {
        match self {
            Update::Cursor(update) =>
                Context { cursor: update.apply(current.cursor), .. current },
            Update::Window(update) =>
                Context { window: update.apply(current.window), .. current }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Update;
    use ::{WindowProperties, Context, Cursor};

    fn default<T: Default>() -> T { Default::default() }

    #[test]
    fn changes_window_position_upon_update() {
        use super::WindowUpdate::MoveTo;
        assert_eq!(
            Update::Window(MoveTo(880, -200))
                .apply(Context {
                    window: WindowProperties { position: (80, -123), .. default() },
                    .. default()
                })
                .window.position,
            (880, -200)
        );
    }

    #[test]
    fn changes_window_size_upon_update() {
        use super::WindowUpdate::Resize;
        assert_eq!(
            Resize(320, 240)
                .apply(WindowProperties { size: (640, 480), .. default() })
                .size,
            (320, 240)
        );
    }

    #[test]
    fn changes_cursor_position_upon_update() {
        use super::CursorUpdate::MoveTo;
        assert_eq!(
            Update::Cursor(MoveTo(50.0, 320.0))
                .apply(Context {
                    cursor: Cursor { position: (0.0, 0.0), .. default() },
                    .. default()
                })
                .cursor.position,
            (50.0, 320.0)
        );
    }

    #[test]
    fn integrates_wheel_position_deltas() {
        use super::CursorUpdate::WheelDelta;
        assert_eq!(
            WheelDelta(3.0, -2.0)
                .apply(Cursor { wheel: (2.0, 1.0), .. default() })
                .wheel,
            (5.0, -1.0)
        );
    }

    #[test]
    fn toggles_window_focus_upon_update() {
        use super::WindowUpdate::Focus;
        assert!(
            Focus(true)
                .apply(WindowProperties { focus: false, .. default() })
                .focus
        );
    }
}
