use font_kit::{font::Font, handle::Handle};
use maschine::{get_device, Event, EventContext};
use raqote::{DrawOptions, DrawTarget, LineCap, PathBuilder, SolidSource, Source, StrokeStyle, Point, AntialiasMode};
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("fonts/Andale_Mono.ttf");

fn main() {
    let font = Handle::from_memory(Arc::new(FONT_BYTES.into()), 0)
        .load()
        .unwrap();

    let mut ctlr = get_device().unwrap();

    let mut encoder_states: [f32; 9] = [0.5; 9];
    let mut should_draw: [bool; 2] = [true; 2];
    loop {
        // Allow controller to do work and update any events
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        // Handle any generated events
        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            match event {
                Event::Encoder(idx, _value, delta, _shift) => {
                    let range = if idx == 0 { 16. } else { 1024. };
                    encoder_states[idx as usize] =
                        (encoder_states[idx as usize] + delta as f32 / range).clamp(0., 1.);
                    if (1..=4).contains(&idx) {
                        should_draw[0] = true;
                    } else if (5..=8).contains(&idx) {
                        should_draw[1] = true;
                    }
                }
                _ => {}
            }
        }
        for (i, d) in should_draw.iter().enumerate() {
            if *d {
                draw(
                    &mut ctlr.get_display(i as u8).unwrap(),
                    &encoder_states[(1 + 4 * i)..=(4 + 4 * i)]
                        .try_into()
                        .unwrap(),
                    &font,
                );
            }
        }
        should_draw = [false; 2];
    }
}

fn draw(dt: &mut DrawTarget, encoder_states: &[f32; 4], font: &Font) {
    const RADIUS: f32 = 20.;
    const OFFSET: f32 = 10.;
    const ANGLE_EXTENT: f32 = std::f32::consts::PI * 0.75;
    const START_ANGLE: f32 = std::f32::consts::PI * 1.5 - ANGLE_EXTENT;

    let src = Source::Solid(SolidSource {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    });

    let draw_opts = DrawOptions {
        antialias: AntialiasMode::None,
        ..Default::default()
    };

    let spacing: f32 = (dt.width() as f32 - 2. * OFFSET - 2. * RADIUS) / 3.;
    dt.clear(SolidSource {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0x00,
    });

    let font_metrics = font.metrics();
    let font_char_width = font_metrics.bounding_box.width() / font_metrics.units_per_em as f32;

    for (i, state) in encoder_states.iter().enumerate() {
        let mut pb = PathBuilder::new();
        let x = OFFSET + RADIUS + spacing * i as f32;
        let y = dt.height() as f32 / 2.;

        pb.arc(x, y, RADIUS, START_ANGLE, 2. * ANGLE_EXTENT * state);

        let path = pb.finish();

        dt.stroke(
            &path,
            &src,
            &StrokeStyle {
                cap: LineCap::Butt,
                width: 4f32,
                ..Default::default()
            },
            &draw_opts
        );

        dt.draw_text(font, 15., format!("E{i}").as_str(), Point::new(x - font_char_width * 15., y + 7.) , &src, &draw_opts);
    }
}
