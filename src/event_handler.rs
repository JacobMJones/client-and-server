use gilrs::{Gilrs, Event, EventType, Axis};
use crate::player::Player;

pub struct EventHandler {
    gilrs: Gilrs,
}

impl EventHandler {
    pub fn new(gilrs: Gilrs) -> Self {
        EventHandler { gilrs }
    }
    pub fn process_events(&mut self, player: &mut Player) {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            if let EventType::AxisChanged(axis, value, ..) = event {
                match axis {
                    Axis::LeftStickX => player.axis_left.0 = value,
                    Axis::LeftStickY => player.axis_left.1 = -value,
                    // Axis::RightStickX => player.axis_right.0 = value,
                    // Axis::RightStickY => player.axis_right.1 = -value,
                    _ => (),
                }
            }
        }
    }
}