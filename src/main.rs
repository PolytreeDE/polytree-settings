use gtk::prelude::*;
use gtk::*;

static ID: &str = "com.causa-arcana.polytree.polytree-session";

fn main() {
    let app = Application::builder().application_id(ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello, World!")
        .build();

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello, World!");
    });

    window.set_child(Some(&button));

    window.present();
}
