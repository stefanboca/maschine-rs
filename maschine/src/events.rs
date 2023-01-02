use crate::error::Error;
use std::collections::VecDeque;

///
/// System Events
///
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Event {
    ///
    /// Button change (Button, Pressed, Shift)
    ///
    Button(Button, bool, bool),

    ///
    /// Encoder change (Encoder Number, value, delta, Shift)
    ///
    Encoder(u8, u16, i16, bool),

    ///
    /// Pad change (Pad Number, Velocity, Shift)
    ///
    Pad(u8, u8, bool),
}

///
/// Button Identifiers
///
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Button {
    Display1,
    Display2,
    Display3,
    Display4,
    Display5,
    Display6,
    Display7,
    Display8,
    Erase,
    Rec,
    Play,
    Grid,
    TransportRight,
    TransportLeft,
    Restart,
    MainEncoder,
    NoteRepeat,
    Sampling,
    Browse,
    All,
    Autowrite,
    Volume,
    Swing,
    Tempo,
    Enter,
    Group,
    GroupA,
    GroupB,
    GroupC,
    GroupD,
    GroupE,
    GroupF,
    GroupG,
    GroupH,
    Main,
    BrowseRight,
    BrowseLeft,
    MainRight,
    MainLeft,
    Nav,
    Control,
    Step,
    F3,
    F2,
    F1,
    Mute,
    Solo,
    Select,
    Duplicate,
    View,
    PadMode,
    Pattern,
    Scene,
    Unknown,
}

///
/// Context object for adding events
///
#[derive(Default)]
pub struct EventContext {
    pub events: VecDeque<Event>,
}

impl EventContext {
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Add an event into the context
    ///
    pub fn add_event(&mut self, event: Event) {
        self.events.push_back(event);
    }
}

///
/// Generator for events
///
pub trait EventTask {
    ///
    /// Perform any update events with the controller device
    ///
    fn tick(&mut self, context: &mut EventContext) -> Result<(), Error>;
}

/// Handle events
pub trait EventHandler {
    /// Handle event and return if it was handled.
    ///
    /// **Arguments**
    /// * `event` - Reference to the event to be handled
    ///
    fn handle(&mut self, event: &Event) -> bool;
}
