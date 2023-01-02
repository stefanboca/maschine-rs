use hidapi::HidApi;
use maschine::{get_device, Colour, Controller, Event, EventContext, EventTask};

fn main() {
    let hid_api = HidApi::new().unwrap();
    let mut ctlr = get_device(&hid_api).unwrap();

    loop {
        // Allow controller to do work and update any events
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        // Handle any generated events
        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            match event {
                Event::Button(button, pressed, shift) => {
                    if pressed {
                        ctlr.set_button_led(button, Colour::random());
                        println!("{:?}", button);
                    } else if !shift {
                        ctlr.set_button_led(button, Colour::BLACK);
                    }
                }
                Event::Pad(pad, velocity, _shift) => {
                    ctlr.set_pad_led(
                        pad,
                        if velocity != 0 {
                            Colour::new(velocity, 0, 0)
                        } else {
                            Colour::BLACK
                        },
                    );
                }
                _ => println!("{:?}", event),
            }
        }
    }
}
