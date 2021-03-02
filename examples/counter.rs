use fltk::*;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default()
        .with_size(200, 50)
        .with_label("Counter");
    let mut pack = group::Pack::default().with_size(180, 40).center_of(&win);
    pack.set_type(group::PackType::Horizontal);
    pack.set_spacing(15);
    let out = output::Output::default().with_size(80, 0);
    out.set_value("0");
    let mut but = button::Button::default()
        .with_size(80, 0)
        .with_label("Count");
    pack.end();
    win.end();
    win.show();

    but.set_callback(move || {
        let mut curr: i32 = out.value().parse().unwrap();
        curr += 1;
        out.set_value(&format!("{}", curr));
    });

    app.run().unwrap();
}
