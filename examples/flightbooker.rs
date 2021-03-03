extern crate chrono;
extern crate regex;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use fltk::*;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap();
}

fn main() {
    let local: DateTime<Local> = Local::now();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(200, 200);
    let mut pack = group::Pack::default().with_size(180, 180).center_of(&win);
    pack.set_spacing(5);
    let mut choice = menu::Choice::default().with_size(0, 40);
    choice.add_choice("one-way flight|return flight");
    choice.set_value(0);
    let mut dep = input::Input::default().with_size(0, 40);
    dep.set_value(&format!(
        "{:02}.{:02}.{:04}",
        local.day(),
        local.month(),
        local.year()
    ));
    dep.set_trigger(CallbackTrigger::Changed);
    let mut ret = input::Input::default().with_size(0, 40);
    ret.set_value(&format!(
        "{:02}.{:02}.{:04}",
        local.day(),
        local.month(),
        local.year()
    ));
    ret.set_trigger(CallbackTrigger::Changed);
    ret.deactivate();
    let mut but = button::Button::default()
        .with_size(0, 40)
        .with_label("Book");
    pack.end();
    win.end();
    win.show();

    choice.set_callback2({
        let mut ret = ret.clone();
        move |c| {
            if c.value() == 0 {
                ret.deactivate();
            } else {
                ret.activate();
            }
        }
    });

    dep.set_callback2({
        let mut but = but.clone();
        let mut ret = ret.clone();
        let choice = choice.clone();
        move |d| {
            let date = d.value();
            if !RE.is_match(&date) {
                d.set_color(Color::Red);
                but.deactivate();
                ret.deactivate();
                app::redraw();
            } else {
                let caps = RE.captures(&date).unwrap();
                let day: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let month: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let year: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                let opt = NaiveDate::from_ymd_opt(year, month, day);
                if opt.is_some() {
                    d.set_color(Color::White);
                    but.activate();
                    if choice.value() == 1 {
                        ret.activate();
                    }
                    app::redraw();
                }
            }
        }
    });

    ret.set_callback2({
        let mut but = but.clone();
        let dep = dep.clone();
        move |r| {
            let date = r.value();
            let dep_date = dep.value();
            if !RE.is_match(&date) {
                r.set_color(Color::Red);
                but.deactivate();
                app::redraw();
            } else {
                let caps = RE.captures(&date).unwrap();
                let day: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let month: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let year: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                let opt = NaiveDate::from_ymd_opt(year, month, day);
                if opt.is_some() {
                    let dep_caps = RE.captures(&dep_date).unwrap();
                    let dep_day: u32 = dep_caps.get(1).unwrap().as_str().parse().unwrap();
                    let dep_month: u32 = dep_caps.get(2).unwrap().as_str().parse().unwrap();
                    let dep_year: i32 = dep_caps.get(3).unwrap().as_str().parse().unwrap();
                    if dep_day <= day && dep_month <= month && dep_year <= year {
                        r.set_color(Color::White);
                        but.activate();
                        app::redraw();
                    }
                }
            }
        }
    });

    but.set_callback(move || {
        let ticket = if choice.value() == 0 {
            "one-way ticket"
        } else {
            "return ticket"
        };
        let dep_date = dep.value();
        let ret_date = ret.value();
        let mut message = format!("You have chosen a {} departing on {}", ticket, dep_date);
        if !ret_date.is_empty() {
            message.push_str(&format!(" and returning on {}", ret_date));
        }
        dialog::message_default(&message);
    });

    app.run().unwrap();
}
