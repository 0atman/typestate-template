/// Based on: <https://zerotomastery.io/blog/rust-typestate-patterns/>
use std::marker::PhantomData;

trait SignalState {}

#[derive(Debug)]
struct Red;
#[derive(Debug)]
struct Yellow;
#[derive(Debug)]
struct Green;
#[derive(Debug)]
struct Error;

impl SignalState for Red {}
impl SignalState for Yellow {}
impl SignalState for Green {}
impl SignalState for Error {}

impl<S: SignalState> TrafficSignal<S> {
    // Helper function to perform state transitions.
    fn transition<T: SignalState>(old: TrafficSignal<T>) -> Self {
        Self {
            _marker: PhantomData,
            data: old.data,
        }
    }
    fn error(self) -> TrafficSignal<Error> {
        TrafficSignal::<Error> {
            _marker: PhantomData,
            data: self.data,
        }
    }
}

#[derive(Debug)]
/// Core state struct
struct TrafficSignal<S: SignalState> {
    _marker: PhantomData<S>,
    data: String,
}

/// NOTE: `default()` must exist in the context of an
/// existing state before returning the initial state,
/// it doesn't matter which state, so we arbitarily choose Red.
///
/// WARN: Red states can be 'reset' with `default()`
impl Default for TrafficSignal<Red> {
    /// Starts in Red state, seems safe
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            data: "INITIAL".into(),
        }
    }
}

impl TrafficSignal<Red> {
    /// Red -> Green
    pub fn next(self) -> TrafficSignal<Green> {
        TrafficSignal::transition(self)
    }
}

impl TrafficSignal<Green> {
    /// Green -> Yellow
    pub fn next(self) -> TrafficSignal<Yellow> {
        TrafficSignal::transition(self)
    }
}

impl TrafficSignal<Yellow> {
    /// Yellow -> Red
    pub fn next(self) -> TrafficSignal<Red> {
        TrafficSignal::transition(self)
    }
}

fn main() {
    let signal = TrafficSignal::default().next().next().next().error();
    dbg!(&signal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation() {
        let signal = TrafficSignal::default();
        let signal = signal.next();
        let signal = signal.next();
        let signal = signal.next();
        let signal = signal.next();
        let signal = signal.next();
        dbg!(&signal);
    }
    #[test]
    fn failure() {
        let signal = TrafficSignal::default();
        let _failed = signal.error();
    }
}
