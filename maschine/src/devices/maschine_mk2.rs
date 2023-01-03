use hidapi::{HidApi, HidDevice};

use crate::colour::Colour;
use crate::display::{Canvas, MonochromeCanvas};
use crate::error::Error;
use crate::events::{Button, Event, EventContext, EventTask};
use crate::{Device, MonoPixel};

const INPUT_BUFFER_SIZE: usize = 512;

// LEDs
pub const LED_CONTROL: u8 = 0;
pub const LED_STEP: u8 = 1;
pub const LED_BROWSE: u8 = 2;
pub const LED_SAMPLING: u8 = 3;
pub const LED_BROWSE_LEFT: u8 = 4;
pub const LED_BROWSE_RIGHT: u8 = 5;
pub const LED_ALL: u8 = 6;
pub const LED_AUTOWRITE: u8 = 7;
pub const LED_DISPLAY1: u8 = 8;
pub const LED_DISPLAY2: u8 = 9;
pub const LED_DISPLAY3: u8 = 10;
pub const LED_DISPLAY4: u8 = 11;
pub const LED_DISPLAY5: u8 = 12;
pub const LED_DISPLAY6: u8 = 13;
pub const LED_DISPLAY7: u8 = 14;
pub const LED_DISPLAY8: u8 = 15;
pub const LED_SCENE: u8 = 16;
pub const LED_PATTERN: u8 = 17;
pub const LED_PADMODE: u8 = 18;
pub const LED_NAVIGATE: u8 = 19;
pub const LED_DUPLICATE: u8 = 20;
pub const LED_SELECT: u8 = 21;
pub const LED_SOLO: u8 = 22;
pub const LED_MUTE: u8 = 23;
pub const LED_VOLUME: u8 = 24;
pub const LED_SWING: u8 = 25;
pub const LED_TEMPO: u8 = 26;
pub const LED_MAIN_LEFT: u8 = 27;
pub const LED_MAIN_RIGHT: u8 = 28;
pub const LED_ENTER: u8 = 29;
pub const LED_NOTE_REPEAT: u8 = 30;
pub const LED_RESTART: u8 = 48;
pub const LED_TRANSPORT_LEFT: u8 = 49;
pub const LED_TRANSPORT_RIGHT: u8 = 50;
pub const LED_GRID: u8 = 51;
pub const LED_PLAY: u8 = 52;
pub const LED_REC: u8 = 53;
pub const LED_ERASE: u8 = 54;
pub const LED_SHIFT: u8 = 55;

pub const LED_PAD13: u8 = 56;
pub const LED_PAD14: u8 = 59;
pub const LED_PAD15: u8 = 62;
pub const LED_PAD16: u8 = 65;
pub const LED_PAD09: u8 = 68;
pub const LED_PAD10: u8 = 71;
pub const LED_PAD11: u8 = 74;
pub const LED_PAD12: u8 = 77;
pub const LED_PAD05: u8 = 80;
pub const LED_PAD06: u8 = 83;
pub const LED_PAD07: u8 = 86;
pub const LED_PAD08: u8 = 89;
pub const LED_PAD01: u8 = 92;
pub const LED_PAD02: u8 = 95;
pub const LED_PAD03: u8 = 98;
pub const LED_PAD04: u8 = 101;

pub const LED_GROUPA: u8 = 104;
pub const LED_GROUPB: u8 = 110;
pub const LED_GROUPC: u8 = 116;
pub const LED_GROUPD: u8 = 122;
pub const LED_GROUPE: u8 = 128;
pub const LED_GROUPF: u8 = 134;
pub const LED_GROUPG: u8 = 140;
pub const LED_GROUPH: u8 = 146;
// pub const LED_UNKNOWN: u8 = 152;

// Buttons

pub const BUTTON_DISPLAY1: u8 = 0;
pub const BUTTON_DISPLAY2: u8 = 1;
pub const BUTTON_DISPLAY3: u8 = 2;
pub const BUTTON_DISPLAY4: u8 = 3;
pub const BUTTON_DISPLAY5: u8 = 4;
pub const BUTTON_DISPLAY6: u8 = 5;
pub const BUTTON_DISPLAY7: u8 = 6;
pub const BUTTON_DISPLAY8: u8 = 7;
pub const BUTTON_CONTROL: u8 = 8;
pub const BUTTON_STEP: u8 = 9;
pub const BUTTON_BROWSE: u8 = 10;
pub const BUTTON_SAMPLING: u8 = 11;
pub const BUTTON_BROWSELEFT: u8 = 12;
pub const BUTTON_BROWSERIGHT: u8 = 13;
pub const BUTTON_ALL: u8 = 14;
pub const BUTTON_AUTOWRITE: u8 = 15;
pub const BUTTON_VOLUME: u8 = 16;
pub const BUTTON_SWING: u8 = 17;
pub const BUTTON_TEMPO: u8 = 18;
pub const BUTTON_MAIN_LEFT: u8 = 19;
pub const BUTTON_MAIN_RIGHT: u8 = 20;
pub const BUTTON_ENTER: u8 = 21;
pub const BUTTON_NOTEREPEAT: u8 = 22;
pub const BUTTON_MAIN_ENCODER: u8 = 23;
pub const BUTTON_GROUPA: u8 = 24;
pub const BUTTON_GROUPB: u8 = 25;
pub const BUTTON_GROUPC: u8 = 26;
pub const BUTTON_GROUPD: u8 = 27;
pub const BUTTON_GROUPE: u8 = 28;
pub const BUTTON_GROUPF: u8 = 29;
pub const BUTTON_GROUPG: u8 = 30;
pub const BUTTON_GROUPH: u8 = 31;
pub const BUTTON_RESTART: u8 = 32;
pub const BUTTON_TRANSPORTLEFT: u8 = 33;
pub const BUTTON_TRANSPORTRIGHT: u8 = 34;
pub const BUTTON_GRID: u8 = 35;
pub const BUTTON_PLAY: u8 = 36;
pub const BUTTON_REC: u8 = 37;
pub const BUTTON_ERASE: u8 = 38;
pub const BUTTON_SHIFT: u8 = 39;
pub const BUTTON_SCENE: u8 = 40;
pub const BUTTON_PATTERN: u8 = 41;
pub const BUTTON_PADMODE: u8 = 42;
pub const BUTTON_NAVIGATE: u8 = 43;
pub const BUTTON_DUPLICATE: u8 = 44;
pub const BUTTON_SELECT: u8 = 45;
pub const BUTTON_SOLO: u8 = 46;
pub const BUTTON_MUTE: u8 = 47;

const BUTTON_COUNT: usize = 48;
const PAD_COUNT: usize = 16;
const ENCODER_COUNT: usize = 9;

const DISPLAY_COUNT: u8 = 2;
const DISPLAY_ADDR: u8 = 0xE0;

const BUTTON_LED_ADDR: u8 = 0x82;
const GROUP_LED_ADDR: u8 = 0x81;
const PAD_LED_ADDR: u8 = 0x80;

const BUTTON_LED_COUNT: usize = 32;
const GROUP_LED_COUNT: usize = 57;
const PAD_LED_COUNT: usize = 49;
///
/// Maschine Mk2 Controller
///
/// Requires a valid HID device
///
pub struct MaschineMk2 {
    device: HidDevice,
    tick_state: u8,
    displays: [MonochromeCanvas; DISPLAY_COUNT as usize],

    button_leds: [u8; BUTTON_LED_COUNT],
    button_leds_dirty: bool,

    group_leds: [u8; GROUP_LED_COUNT],
    group_leds_dirty: bool,

    pad_leds: [u8; PAD_LED_COUNT],
    pad_leds_dirty: bool,

    button_states: [bool; BUTTON_COUNT],
    shift_pressed: bool,
    pads_data: [u16; PAD_COUNT],
    pads_status: [bool; PAD_COUNT],
    encoder_values: [u16; ENCODER_COUNT],
}

impl MaschineMk2 {
    pub const VENDOR_ID: u16 = 0x17cc;
    pub const PRODUCT_ID: u16 = 0x1140;

    /// Send a display frame for the graphics panel
    fn send_frame(&mut self, display_idx: u8) -> Result<(), Error> {
        if display_idx >= DISPLAY_COUNT {
            return Err(Error::InvalidDisplay(display_idx));
        }
        if self.displays[display_idx as usize].is_dirty() {
            for chunk in 0..8 {
                // The number of referenced bytes must be <= 256
                // Eg Column width * number of rows
                let mut buffer: Vec<u8> = vec![
                    DISPLAY_ADDR | display_idx,
                    0x00,              // Column offset
                    0x00,              // ?
                    (chunk * 8) as u8, // Row (a row is 8 pixels high)
                    0x00,              // ?
                    0x20,              // Columns per row, 128 is full width
                    0x00,              // ?
                    0x08,              // Number of rows
                    0x00,              // ?
                ];
                let x_offset = chunk * 256;
                buffer.extend_from_slice(
                    &self.displays[display_idx as usize].data()[x_offset..(x_offset + 256)],
                );
                self.device.write(buffer.as_slice())?;
            }
            self.displays[display_idx as usize].clear_dirty_flag();
        }

        Ok(())
    }

    /// Update LEDs if the array has been updated
    fn send_leds(&mut self) -> Result<(), Error> {
        if self.button_leds_dirty {
            let mut buffer: Vec<u8> = vec![BUTTON_LED_ADDR];
            buffer.extend_from_slice(&self.button_leds);
            self.device.write(buffer.as_slice())?;
            self.button_leds_dirty = false;
        }
        if self.group_leds_dirty {
            let mut buffer: Vec<u8> = vec![GROUP_LED_ADDR];
            buffer.extend_from_slice(&self.group_leds);
            self.device.write(buffer.as_slice())?;
            self.group_leds_dirty = false;
        }
        if self.pad_leds_dirty {
            let mut buffer: Vec<u8> = vec![PAD_LED_ADDR];
            buffer.extend_from_slice(&self.pad_leds);
            self.device.write(buffer.as_slice())?;
            self.pad_leds_dirty = false;
        }

        Ok(())
    }

    /// Read incoming reports from the device
    fn read(&mut self, context: &mut EventContext) -> Result<(), Error> {
        let mut buffer = [0u8; INPUT_BUFFER_SIZE];

        for idx in 0..32 {
            let bytes_read = match self.device.read(&mut buffer) {
                Ok(n) => n,
                Err(e) => return Err(Error::HidAPI(e)),
            };

            if bytes_read > 0 && buffer[0] == 0x01 {
                self.process_buttons(&buffer[1..25], context)?;
            } else if (bytes_read > 0) && (buffer[0] == 0x20) && ((idx % 7) == 0) {
                self.process_pads(&buffer[1..], context)?;
            }
        }

        Ok(())
    }

    /// Process a buttons report message
    fn process_buttons(&mut self, buffer: &[u8], context: &mut EventContext) -> Result<(), Error> {
        if buffer.len() < 24 {
            return Err(Error::InvalidReport);
        }

        fn is_button_pressed(buffer: &[u8], button: u8) -> bool {
            let byte_idx = (button >> 3) as usize;
            (buffer[byte_idx] & (1 << (button % 8))) != 0
        }

        // Scan buttons
        for btn in BUTTON_DISPLAY1..=BUTTON_MUTE {
            let button_pressed = is_button_pressed(buffer, btn);
            if button_pressed != self.button_states[btn as usize] {
                // println!("{btn}, {button_pressed}");
                self.button_states[btn as usize] = button_pressed;

                if btn == BUTTON_SHIFT {
                    self.shift_pressed = button_pressed;
                    self.set_led(
                        LED_SHIFT,
                        if button_pressed {
                            Colour::WHITE
                        } else {
                            Colour::BLACK
                        },
                    );
                } else {
                    let button = self.as_device_button(btn);
                    context.add_event(Event::Button(button, button_pressed, self.shift_pressed));
                }
            }
        }

        // Handle encoder data
        // TODO: encoder initialization / first event delta should be zero, also for Mikro MK2
        for i in 0..ENCODER_COUNT {
            // Encoder 0 is the main encoder, hence having a different range and value format

            // The encoder value will all ways be in the rance 0 <= value < encoder_range
            let encoder_range = if i == 0 { 0x10 } else { 0x400 };

            let value = if i == 0 {
                buffer[7] as u16
            } else {
                ((buffer[7 + 2 * i] as u16) << 8) + buffer[6 + 2 * i] as u16
            };

            if self.encoder_values[i] != value {
                // TODO: explainer comment
                let pos_delta =
                    ((encoder_range + value - self.encoder_values[i]) % encoder_range) as i16;
                let neg_delta =
                    ((encoder_range - value + self.encoder_values[i]) % encoder_range) as i16;

                let delta = if pos_delta < neg_delta {
                    pos_delta
                } else {
                    -neg_delta
                };

                context.add_event(Event::Encoder(i as u8, value, delta, self.shift_pressed));
                self.encoder_values[i] = value;
            }
        }

        Ok(())
    }

    /// Process a pads report message
    fn process_pads(&mut self, buffer: &[u8], context: &mut EventContext) -> Result<(), Error> {
        if buffer.len() < 64 {
            return Err(Error::InvalidReport);
        }

        for idx in (0..32).step_by(2) {
            let low_byte = buffer[idx];
            let high_byte = buffer[idx + 1];
            let pad = ((high_byte & 0xF0) >> 4) as usize;
            let value = (((high_byte & 0x0F) as u16) << 8) | low_byte as u16;
            let pressed = value > 512;

            self.pads_data[pad] = value;
            if pressed | self.pads_status[pad] {
                self.pads_status[pad] = pressed;
                context.add_event(Event::Pad(
                    pad as u8,
                    if pressed { (value >> 4) as u8 } else { 0 },
                    self.shift_pressed,
                ));
            }
        }

        Ok(())
    }

    /// Set the colour of an LED
    fn set_led(&mut self, led: u8, colour: Colour) {
        let base = led as usize;

        if self.is_rgb_led(led) {
            let (r, g, b) = colour.components();

            if (LED_PAD13..=LED_PAD04).contains(&led) {
                let pad_base = base - LED_PAD13 as usize;

                self.pad_leds[pad_base] = r;
                self.pad_leds[pad_base + 1] = g;
                self.pad_leds[pad_base + 2] = b;
                self.pad_leds_dirty = true;
            } else if (LED_GROUPA..=LED_GROUPH).contains(&led) {
                let group_base = base - LED_GROUPA as usize;

                self.group_leds[group_base] = r;
                self.group_leds[group_base + 1] = g;
                self.group_leds[group_base + 2] = b;
                self.group_leds[group_base + 3] = r;
                self.group_leds[group_base + 4] = g;
                self.group_leds[group_base + 5] = b;
                self.group_leds_dirty = true;
            }
        } else {
            let m = colour.as_1bit();
            if led >= LED_RESTART {
                self.group_leds[base] = m;
                self.group_leds_dirty = true;
            } else {
                self.button_leds[base] = m;
                self.button_leds_dirty = true;
            }
        }
    }

    /// Determine if an LED is RGB or Mono
    fn is_rgb_led(&self, led: u8) -> bool {
        // FIXME: group buttons have two LEDs
        (LED_GROUPA..=LED_GROUPH).contains(&led) | (LED_PAD13..=LED_PAD04).contains(&led)
    }

    /// Convert a button code into a button enum
    fn as_device_button(&self, button: u8) -> Button {
        match button {
            BUTTON_DISPLAY1 => Button::Display1,
            BUTTON_DISPLAY2 => Button::Display2,
            BUTTON_DISPLAY3 => Button::Display3,
            BUTTON_DISPLAY4 => Button::Display4,
            BUTTON_DISPLAY5 => Button::Display5,
            BUTTON_DISPLAY6 => Button::Display6,
            BUTTON_DISPLAY7 => Button::Display7,
            BUTTON_DISPLAY8 => Button::Display8,
            BUTTON_CONTROL => Button::Control,
            BUTTON_STEP => Button::Step,
            BUTTON_BROWSE => Button::Browse,
            BUTTON_SAMPLING => Button::Sampling,
            BUTTON_BROWSELEFT => Button::BrowseLeft,
            BUTTON_BROWSERIGHT => Button::BrowseRight,
            BUTTON_ALL => Button::All,
            BUTTON_AUTOWRITE => Button::Autowrite,
            BUTTON_VOLUME => Button::Volume,
            BUTTON_SWING => Button::Swing,
            BUTTON_TEMPO => Button::Tempo,
            BUTTON_MAIN_LEFT => Button::MainLeft,
            BUTTON_MAIN_RIGHT => Button::MainRight,
            BUTTON_ENTER => Button::Enter,
            BUTTON_NOTEREPEAT => Button::NoteRepeat,
            BUTTON_MAIN_ENCODER => Button::MainEncoder,
            BUTTON_GROUPA => Button::GroupA,
            BUTTON_GROUPB => Button::GroupB,
            BUTTON_GROUPC => Button::GroupC,
            BUTTON_GROUPD => Button::GroupD,
            BUTTON_GROUPE => Button::GroupE,
            BUTTON_GROUPF => Button::GroupF,
            BUTTON_GROUPG => Button::GroupG,
            BUTTON_GROUPH => Button::GroupH,
            BUTTON_RESTART => Button::Restart,
            BUTTON_TRANSPORTLEFT => Button::TransportLeft,
            BUTTON_TRANSPORTRIGHT => Button::TransportRight,
            BUTTON_GRID => Button::Grid,
            BUTTON_PLAY => Button::Play,
            BUTTON_REC => Button::Rec,
            BUTTON_ERASE => Button::Erase,
            BUTTON_SCENE => Button::Scene,
            BUTTON_PATTERN => Button::Pattern,
            BUTTON_PADMODE => Button::PadMode,
            BUTTON_NAVIGATE => Button::Nav,
            BUTTON_DUPLICATE => Button::Duplicate,
            BUTTON_SELECT => Button::Select,
            BUTTON_SOLO => Button::Solo,
            BUTTON_MUTE => Button::Mute,
            _ => Button::Unknown,
        }
    }

    /// Convert a button into a LED index
    fn button_to_led(&self, button: Button) -> Option<u8> {
        match button {
            Button::Erase => Some(LED_ERASE),
            Button::Rec => Some(LED_REC),
            Button::Play => Some(LED_PLAY),
            Button::Grid => Some(LED_GRID),
            Button::TransportRight => Some(LED_TRANSPORT_RIGHT),
            Button::TransportLeft => Some(LED_TRANSPORT_LEFT),
            Button::Restart => Some(LED_RESTART),
            Button::NoteRepeat => Some(LED_NOTE_REPEAT),
            Button::Sampling => Some(LED_SAMPLING),
            Button::Browse => Some(LED_BROWSE),
            Button::GroupA => Some(LED_GROUPA),
            Button::GroupB => Some(LED_GROUPB),
            Button::GroupC => Some(LED_GROUPC),
            Button::GroupD => Some(LED_GROUPD),
            Button::GroupE => Some(LED_GROUPE),
            Button::GroupF => Some(LED_GROUPF),
            Button::GroupG => Some(LED_GROUPG),
            Button::GroupH => Some(LED_GROUPH),
            Button::BrowseRight => Some(LED_BROWSE_RIGHT),
            Button::BrowseLeft => Some(LED_BROWSE_LEFT),
            Button::Nav => Some(LED_NAVIGATE),
            Button::Control => Some(LED_CONTROL),
            Button::Display1 => Some(LED_DISPLAY1),
            Button::Display2 => Some(LED_DISPLAY2),
            Button::Display3 => Some(LED_DISPLAY3),
            Button::Display4 => Some(LED_DISPLAY4),
            Button::Display5 => Some(LED_DISPLAY5),
            Button::Display6 => Some(LED_DISPLAY6),
            Button::Display7 => Some(LED_DISPLAY7),
            Button::Display8 => Some(LED_DISPLAY8),
            Button::Mute => Some(LED_MUTE),
            Button::Solo => Some(LED_SOLO),
            Button::Select => Some(LED_SELECT),
            Button::Duplicate => Some(LED_DUPLICATE),
            Button::PadMode => Some(LED_PADMODE),
            Button::Pattern => Some(LED_PATTERN),
            Button::Scene => Some(LED_SCENE),
            Button::All => Some(LED_ALL),
            Button::Autowrite => Some(LED_AUTOWRITE),
            Button::Volume => Some(LED_VOLUME),
            Button::Swing => Some(LED_SWING),
            Button::Tempo => Some(LED_TEMPO),
            Button::Enter => Some(LED_ENTER),
            Button::Step => Some(LED_STEP),
            Button::MainLeft => Some(LED_MAIN_LEFT),
            Button::MainRight => Some(LED_MAIN_RIGHT),
            _ => None,
        }
    }

    /// Convert a button into a LED index
    fn pad_to_led(&self, pad: u8) -> Option<u8> {
        match pad {
            0x0 => Some(LED_PAD13),
            0x1 => Some(LED_PAD14),
            0x2 => Some(LED_PAD15),
            0x3 => Some(LED_PAD16),
            0x4 => Some(LED_PAD09),
            0x5 => Some(LED_PAD10),
            0x6 => Some(LED_PAD11),
            0x7 => Some(LED_PAD12),
            0x8 => Some(LED_PAD05),
            0x9 => Some(LED_PAD06),
            0xA => Some(LED_PAD07),
            0xB => Some(LED_PAD08),
            0xC => Some(LED_PAD01),
            0xD => Some(LED_PAD02),
            0xE => Some(LED_PAD03),
            0xF => Some(LED_PAD04),
            _ => None,
        }
    }
}

impl Device for MaschineMk2 {
    type Pixel = MonoPixel;

    fn new() -> Result<Self, Error> {
        let hid_api = HidApi::new()?;
        Ok(MaschineMk2 {
            device: hid_api.open(MaschineMk2::VENDOR_ID, MaschineMk2::PRODUCT_ID)?,
            tick_state: 0,
            displays: [
                MonochromeCanvas::new(256, 64),
                MonochromeCanvas::new(256, 64),
            ],

            button_leds: [0; BUTTON_LED_COUNT],
            button_leds_dirty: true,

            group_leds: [0; GROUP_LED_COUNT],
            group_leds_dirty: true,

            pad_leds: [0; PAD_LED_COUNT],
            pad_leds_dirty: true,

            button_states: [false; BUTTON_COUNT],
            shift_pressed: false,
            pads_data: [0; PAD_COUNT],
            pads_status: [false; PAD_COUNT],
            encoder_values: [0; ENCODER_COUNT],
        })
    }

    fn set_button_led(&mut self, button: Button, colour: Colour) {
        if let Some(led) = self.button_to_led(button) {
            self.set_led(led, colour);
        }
    }

    fn set_pad_led(&mut self, pad: u8, colour: Colour) {
        if let Some(led) = self.pad_to_led(pad) {
            self.set_led(led, colour);
        }
    }

    fn get_display(&mut self, display_idx: u8) -> Result<Box<&mut dyn Canvas<Self::Pixel>>, Error> {
        if display_idx >= DISPLAY_COUNT {
            Err(Error::InvalidDisplay(display_idx))
        } else {
            Ok(Box::new(&mut self.displays[display_idx as usize]))
        }
    }
}

impl EventTask for MaschineMk2 {
    fn tick(&mut self, context: &mut EventContext) -> Result<(), Error> {
        if self.tick_state == 0 {
            for i in 0..DISPLAY_COUNT {
                self.send_frame(i)?;
            }
        } else if self.tick_state == 1 {
            self.send_leds()?;
        } else if self.tick_state == 2 {
            self.read(context)?;
        }

        self.tick_state = (self.tick_state + 1) % 3;

        Ok(())
    }
}
