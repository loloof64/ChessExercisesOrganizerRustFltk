use fltk::{app::{App,AppScheme}, button::*, window::*, dialog::alert};

fn main() {
    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut window = Window::default()
        .with_size(400, 300)
        .with_label("Chess exercises organizer")
        .center_screen();
    let mut button = Button::default().with_size(70, 20).with_label("Say hello").center_of(&window);
    window.end();
    window.show();

    window.set_color(Color::White);
    button.set_color(Color::Cyan);

    button.set_callback(Box::new(|| {
        alert(10, 10, "Hello !");
    }));

    app.run().expect("Failed to run the application !");
}
