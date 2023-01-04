use maschine::{get_device, Color, Event, EventContext};

fn main() {
    let mut ctlr = get_device().unwrap();

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
                        ctlr.set_button_led(button, Color::new(0xFF, 0x00, 0x00, 0xFF));
                    } else if !shift {
                        ctlr.set_button_led(button, Color::new(0xFF, 0x00, 0x00, 0x00));
                    }
                }
                Event::Pad(pad, velocity, _shift) => {
                    ctlr.set_pad_led(
                        pad,
                        if velocity != 0 {
                            let gamma = 2.0;
                            let brightness = f64::round(
                                (256_f64).powf(1.0 - gamma) * (velocity as f64).powf(gamma),
                            ) as u8;
                            Color::new(0xFF, brightness, 0, 0)
                        } else {
                            Color::new(0xFF, 0x00, 0x00, 0x00)
                        },
                    );
                }
                _ => {}
            }
        }
    }
}
