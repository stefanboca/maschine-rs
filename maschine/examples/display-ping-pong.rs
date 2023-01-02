use hidapi::HidApi;
use maschine::{get_device, Canvas, EventContext, EventTask, Pixel, ScrollDirection};

fn main() {
    let hid_api = HidApi::new().unwrap();
    let mut ctlr = get_device(&hid_api).unwrap();
    ctlr.displays[0].fill_row(7, Pixel::On);
    ctlr.displays[1].fill_row(0, Pixel::On);

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
            ctlr.displays[0].vscroll_rows(0, 7, dir_1);
            ctlr.displays[1].vscroll_rows(0, 7, dir_2);
            step += 1;
            if step % 7 == 0 {
                (dir_1, dir_2) = (dir_2, dir_1);
            }
            start_time = current_time;
        }
    }
}
