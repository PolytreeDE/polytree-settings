use gtk::prelude::*;
use gtk::*;

static ID: &str = "com.causa-arcana.polytree.polytree-session";
static UI: &str = include_str!("../main.ui");

fn main() {
    let app = Application::builder().application_id(ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let builder = Builder::from_string(UI);
    let app_window: ApplicationWindow = builder.object("app-window").unwrap();
    app_window.set_application(Some(app));
    app_window.present();
}
