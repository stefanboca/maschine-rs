use maschine::{Device, EventContext, EventTask, MaschineMk2, Pixel, ScrollDirection};

fn main() {
    let mut ctlr = MaschineMk2::new().unwrap();

    ctlr.get_display(0).unwrap().fill_row(7, Pixel::On);
    ctlr.get_display(1).unwrap().fill_row(0, Pixel::On);

    let mut start_time = std::time::Instant::now();
    let mut step = 0;
    let mut dir_1 = ScrollDirection::Down;
    let mut dir_2 = ScrollDirection::Up;
    loop {
        // Allow controller to do work and update any events
        let mut context = EventContext::new();
        ctlr.tick(&mut context).unwrap();

        let current_time = std::time::Instant::now();
        if current_time.duration_since(start_time) > std::time::Duration::from_millis(200) {
            ctlr.get_display(0).unwrap().vscroll_rows(0, 7, dir_1);
            ctlr.get_display(1).unwrap().vscroll_rows(0, 7, dir_2);
            step += 1;
            if step % 7 == 0 {
                (dir_1, dir_2) = (dir_2, dir_1);
            }
            start_time = current_time;
        }
    }
}
