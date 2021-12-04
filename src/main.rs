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

    window.present();
}
