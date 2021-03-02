use fltk::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let duration = 200.0;
    let timer = timer::Timer::new();
    let (s, r) = app::channel();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let _guard = timer.schedule_repeating(chrono::Duration::milliseconds(100), move || {
        s.send(0.1);
    });
    let mut win = window::Window::default().with_size(400, 200);
    let mut pack = group::Pack::new(120, 10, 250, 130, "");
    pack.set_spacing(10);
    let mut prog = misc::Progress::default()
        .with_size(0, 30)
        .with_label("Elapsed time:")
        .with_align(Align::Left);
    prog.set_maximum(duration);
    let frame = frame::Frame::default()
        .with_size(0, 40)
        .with_label(&format!("{:02.01}s", duration))
        .with_align(Align::Left);
    let mut slider = valuator::HorNiceSlider::default()
        .with_size(0, 20)
        .with_label("Duration:")
        .with_align(Align::Left);
    slider.set_maximum(4.0);
    slider.set_value(duration / 100.0);
    let duration_rc = Rc::from(RefCell::from(duration));
    slider.set_callback2({
        let mut frame = frame.clone();
        let mut prog = prog.clone();
        let dur = duration_rc.clone();
        move |s| {
            let val: f64 = s.value() * 100.;
            *dur.borrow_mut() = val;
            frame.set_label(&format!("{:02.01}s", val));
            prog.set_maximum(val as _);
            app::redraw();
        }
    });
    pack.end();
    let mut but = button::Button::new(10, 140, 380, 40, "Reset");
    but.set_callback2(|b| println!("{}", b.y()));
    win.end();
    win.show();

    let mut start = 0.0;
    while app.wait() {
        if let Some(msg) = r.recv() {
            start += msg;
            if start == *duration_rc.borrow() {
                continue;
            }
            let max = prog.maximum();
            prog.set_value(start);
        }
    }
}
