
use {
    AddInterval,
    AddPress,
    IntervalEvent,
    KeyType,
    PressEvent,
    Value,
};

/// An immutable event context. All Request starting here.
pub struct Event;

impl Event {
    /// Returns a new event context.
    pub fn new() -> Event {
        Event
    }
}

impl<'a> AddPress<'a, PressEvent<'a>> for Event {
    #[inline(always)]
    fn press(&'a self, key: &'a KeyType) -> PressEvent<'a> {
        PressEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddInterval<IntervalEvent<'a>> for Event {
    #[inline(always)]
    fn interval(&self, seconds: f64) -> IntervalEvent<'a> {
        IntervalEvent {
            interval: Value(seconds),
        }
    }
}

