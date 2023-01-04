use maschine::{get_device, EventContext};
use raqote::{DrawOptions, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};

fn main() {
    let mut ctlr = get_device().unwrap();

    let dt = ctlr.get_display(0).unwrap();

    let mut pb = PathBuilder::new();
    pb.rect(10., 10., 236., 44.);
    pb.close();
    pb.arc(128., 32., 20., 0., 2. * std::f32::consts::PI);
    pb.close();
    let path = pb.finish();

    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
            a: 0xFF,
        }),
        &StrokeStyle {
            cap: LineCap::Square,
            join: LineJoin::Bevel,
            width: 2f32,
            miter_limit: 2f32,
            ..Default::default()
        },
        &DrawOptions::new(),
    );

    loop {
        // Allow controller to do work and update any events
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        // Handle any generated events
        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();
            println!("{event:?}");
        }
    }
}
