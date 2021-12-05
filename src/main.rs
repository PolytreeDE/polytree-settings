use std::cell::Cell;
use std::rc::Rc;

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

    let label = Label::builder()
        .label("0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let label1 = label.clone();
    let label2 = label.clone();

    let button1 = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button2 = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number1 = Rc::new(Cell::new(0));
    let number2 = number1.clone();

    button1.connect_clicked(move |_| {
        number1.set(number1.get() + 1);
        label1.set_label(&number1.get().to_string());
    });
    button2.connect_clicked(move |_| {
        number2.set(number2.get() - 1);
        label2.set_label(&number2.get().to_string());
    });

    let box_ = Box::new(Orientation::Vertical, 0);

    box_.append(&label);
    box_.append(&button1);
    box_.append(&button2);

    window.set_child(Some(&box_));

    window.present();
}
