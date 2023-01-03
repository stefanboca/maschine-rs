use maschine::{Colour, Device, Event, EventContext, EventTask, MaschineMk2};

fn main() {
    let mut ctlr = MaschineMk2::new().unwrap();

    loop {
        // Allow controller to do work and update any events
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        // Handle any generated events
        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            println!("{event:?}");
            match event {
                Event::Button(button, pressed, shift) => {
                    if pressed {
                        ctlr.set_button_led(button, Colour::random());
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
                _ => {}
            }
        }
    }
}
