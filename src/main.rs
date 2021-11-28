use gtk::prelude::*;
use gtk::*;

static ID: &str = "com.causa-arcana.polytree.polytree-session";

fn main() {
    let app = Application::builder().application_id(ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);

    window.set_title("Polytree settings");
    window.set_border_width(10);
    window.set_position(WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = Button::with_label("Press me!");

    window.add(&button);

    window.show_all();
}
