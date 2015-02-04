use glutin;
use nalgebra::{one, zero, BaseFloat};
use carboxyl::{Stream, Cell};


/// A button.
///
/// FIXME: replace by own buttons
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Button {
    Glutin(glutin::VirtualKeyCode),
}


/// The state of a button.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl ButtonState {
    /// Track the state of a button in a cell from a stream of button events.
    pub fn track(inputs: &Stream<ButtonEvent>, button: Button)
        -> Cell<ButtonState>
    {
        inputs
            .map(move |event|
                if event.button == button { Some(event.state) }
                else { None }
            )
            .filter()
            .hold(ButtonState::Released)
    }
}


/// A direction indicated by the state of two "opposite" buttons. This could be
/// left-right, forward-back and similar states.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Positive,
    Negative,
    Still,
}

impl Direction {
    /// Derive a direction from two button states.
    pub fn from_buttons(plus: ButtonState, minus: ButtonState)
        -> Direction
    {
        use button::ButtonState::{Pressed, Released};
        match (plus, minus) {
            (Pressed, Released) => Direction::Positive,
            (Released, Pressed) => Direction::Negative,
            _ => Direction::Still,
        }
    }

    /// The sign of a button state as a float.
    pub fn sign<T: BaseFloat>(&self) -> T {
        match *self {
            Direction::Positive => one(),
            Direction::Negative => -one::<T>(),
            Direction::Still => zero(),
        }
    }

    /// Track direction from a stream of button events in a cell.
    pub fn track(inputs: &Stream<ButtonEvent>, pos: Button, neg: Button)
        -> Cell<Direction>
    {
        lift!(
            Direction::from_buttons,
            &ButtonState::track(inputs, pos),
            &ButtonState::track(inputs, neg)
        )
    }
}


/// An event involving a button of some kind.
#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub struct ButtonEvent {
    /// The button concerned.
    pub button: Button,

    /// The new state of the button.
    pub state: ButtonState,
}


#[cfg(test)]
mod test {
    use carboxyl::Sink;
    use super::*;
    use super::ButtonState::{Pressed, Released};

    #[test]
    fn test_track_button_glutin() {
        use glutin::VirtualKeyCode::A;
        let sink = Sink::new();
        let state = ButtonState::track(&sink.stream(), Button::Glutin(A));
        assert_eq!(state.sample(), Released);
        sink.send(ButtonEvent { button: Button::Glutin(A), state: Pressed });
        assert_eq!(state.sample(), Pressed);
        sink.send(ButtonEvent { button: Button::Glutin(A), state: Released });
        assert_eq!(state.sample(), Released);
    }

    #[test]
    fn axis_buttons_glutin() {
        use glutin::VirtualKeyCode::{W, S};
        use super::Direction::*;

        let sink = Sink::new();
        let direction = Direction::track(
            &sink.stream(), Button::Glutin(W), Button::Glutin(S));

        assert_eq!(direction.sample(), Still);

        sink.send(ButtonEvent { button: Button::Glutin(W), state: Pressed });
        assert_eq!(direction.sample(), Positive);

        sink.send(ButtonEvent { button: Button::Glutin(S), state: Pressed });
        assert_eq!(direction.sample(), Still);

        sink.send(ButtonEvent { button: Button::Glutin(W), state: Released });
        assert_eq!(direction.sample(), Negative);

        sink.send(ButtonEvent { button: Button::Glutin(S), state: Released });
        assert_eq!(direction.sample(), Still);
    }
}

