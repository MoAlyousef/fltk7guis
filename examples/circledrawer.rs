use fltk::*;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

pub struct Circle {
    frm: frame::Frame,
}

pub enum DrawEvent {
    DrawCircle,
    ResizeCircle(i32, u32),      // prev_size, idx
    DeleteCircle(i32, i32, i32), // x, y, radius
}

pub struct State(Option<DrawEvent>);

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State(None));
    static ref CURRENT: Mutex<u32> = Mutex::new(0);
}

impl Circle {
    pub fn new(x: i32, y: i32) -> Self {
        let mut frm = frame::Frame::new(x - 10, y - 10, 20, 20, "");
        frm.draw2(move |f| {
            let width = f.width() / 2;
            draw::set_draw_color(f.selection_color());
            draw::draw_pie(f.x(), f.y(), width * 2, width * 2, 0., 360.);
            draw::set_draw_color(Color::Black);
            draw::draw_circle((f.x() + width) as _, (f.y() + width) as _, width as _);
        });
        let mut menu = menu::MenuItem::new(&["Adjust diameter\t ..."]);
        frm.handle2(move |f, ev| match ev {
            Event::Push => {
                if app::event_mouse_button() == Mouse::Right {
                    match menu.popup(x, y) {
                        None => (),
                        Some(_) => {
                            let mut win = window::Window::default().with_size(300, 50);
                            let mut slider = valuator::HorNiceSlider::new(
                                10,
                                30,
                                280,
                                20,
                                &format!("Adjust diameter of circle at ({}, {})", x, y),
                            );
                            slider.set_align(Align::Top);
                            slider.set_minimum(20.);
                            slider.set_maximum(100.);
                            slider.set_value(f.width() as f64);
                            let mut state = STATE.lock().unwrap();
                            f.do_callback();
                            let curr = CURRENT.lock().unwrap();
                            *state = State(Some(DrawEvent::ResizeCircle(f.width(), *curr)));
                            let mut f_c = f.clone();
                            slider.set_callback2(move |s| {
                                let val = s.value() as i32;
                                f_c.resize(f_c.x(), f_c.y(), val, val);
                                f_c.top_window().unwrap().redraw();
                            });
                            win.end();
                            win.make_modal(true);
                            win.show();
                            while win.shown() {
                                app::wait();
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        });
        Circle { frm }
    }
}

impl Deref for Circle {
    type Target = frame::Frame;

    fn deref(&self) -> &Self::Target {
        &self.frm
    }
}

impl DerefMut for Circle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frm
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default()
        .with_size(500, 400)
        .with_label("CircleDraw");
    let mut undo = button::Button::new(160, 10, 80, 30, "Undo");
    let mut redo = button::Button::default()
        .with_label("Redo")
        .size_of(&undo)
        .right_of(&undo, 10);
    let mut group = group::Group::new(5, 45, 490, 350, "");
    group.set_frame(FrameType::DownBox);
    group.set_color(Color::White);
    let mut frame = group::Group::new(15, 55, 470, 325, "");
    frame.set_color(Color::White);
    frame.end();
    group.end();
    win.end();
    win.show();

    frame.handle2(move |f, ev| match ev {
        Event::Push => {
            if app::event_mouse_button() == Mouse::Left {
                let coords = app::event_coords();
                let x = coords.0;
                let y = coords.1;
                if x < f.x() || y < f.y() {
                    return false;
                }
                let mut c = Circle::new(x, y);
                f.add(&*c);
                let idx = f.find(&*c);
                c.set_callback(move || {
                    let mut curr = CURRENT.lock().unwrap();
                    *curr = idx;
                });
                let mut state = STATE.lock().unwrap();
                *state = State(Some(DrawEvent::DrawCircle));
                f.redraw();
                true
            } else {
                false
            }
        }
        Event::Move => {
            for i in 0..f.children() {
                let coords = app::event_coords();
                let x = coords.0;
                let y = coords.1;
                let mut circle = f.child(i).unwrap();
                let radius = circle.width() / 2;
                let circle_x = circle.x() + radius;
                let circle_y = circle.y() + radius;
                let d = {
                    let xd = x as i32 - circle_x as i32;
                    let yd = y as i32 - circle_y as i32;
                    ((xd.pow(2) + yd.pow(2)) as f64).sqrt().powi(2)
                };
                if d < (radius as f64).powi(2) {
                    circle.set_selection_color(Color::from_rgb(200, 200, 200));
                    win.redraw();
                } else {
                    circle.set_selection_color(Color::White);
                    win.redraw();
                }
            }
            true
        }
        _ => false,
    });

    undo.set_callback({
        let frame = frame.clone();
        move || {
            let children = frame.children();
            if children == 0 {
                return;
            }
            let mut state = STATE.lock().unwrap();
            match state.0 {
                Some(DrawEvent::DrawCircle) => {
                    if let Some(child) = frame.child(children - 1) {
                        *state = State(Some(DrawEvent::DeleteCircle(
                            child.x(),
                            child.y(),
                            child.width(),
                        )));
                        unsafe {
                            frame::Frame::delete(frame::Frame::from_widget_ptr(
                                child.as_widget_ptr(),
                            ))
                        };
                    }
                }
                Some(DrawEvent::ResizeCircle(val, idx)) => {
                    if let Some(mut child) = frame.child(idx) {
                        *state = State(Some(DrawEvent::ResizeCircle(child.width(), idx)));
                        child.set_size(val, val);
                    }
                }
                Some(DrawEvent::DeleteCircle(_, _, _)) => (),
                None => (),
            }
            frame.top_window().unwrap().redraw();
        }
    });

    redo.set_callback(move || {
        let mut state = STATE.lock().unwrap();
        match state.0 {
            Some(DrawEvent::DrawCircle) => (),
            Some(DrawEvent::ResizeCircle(val, idx)) => {
                let children = frame.children();
                if children == 0 {
                    return;
                }
                if let Some(mut child) = frame.child(idx) {
                    *state = State(Some(DrawEvent::ResizeCircle(child.width(), idx)));
                    child.set_size(val, val);
                }
            }
            Some(DrawEvent::DeleteCircle(x, y, r)) => {
                *state = State(Some(DrawEvent::DrawCircle));
                let mut c = Circle::new(x, y);
                c.set_size(r, r);
                frame.add(&*c);
                frame.top_window().unwrap().redraw();
            }
            None => (),
        }
        frame.top_window().unwrap().redraw();
    });

    app.run().unwrap();
}
