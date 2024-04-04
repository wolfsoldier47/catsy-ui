use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.catsy-gui-glade"),
        Default::default()
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("gtk.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.object(
        "applicationwindow1"
    ).unwrap();

    let message_input: gtk::Entry = builder.object(
        "message_input"
    ).unwrap();

    let button: gtk::Button = builder.object(
        "generate_btn"
    ).unwrap();

    let message_output: gtk::Label = builder.object(
        "message_output"
    ).unwrap();

    let image_output: gtk::Image = builder.object(
        "image_output"
    ).unwrap();

    let image_output_clone = image_output.clone();
   
    let is_dead_switch: gtk::Switch = builder.object(
        "is_dead_switch"
    ).unwrap();

    let image_output: gtk::Image = builder.object(
        "image_output"
    ).unwrap();
    // let image_output_clone = image_output.clone();
    button.connect_clicked(move |_| {
        message_output.set_text(&format!(
            "{}\n \\\n \\",
            message_input.text().as_str()
        ));
        let is_dead = is_dead_switch.is_active();
        if is_dead {
            image_output_clone.set_from_file(
                Some("./images/cat_dead.png")
            )
        } else {
            image_output_clone.set_from_file(
                Some("./images/cat.png")
            )
        }
        image_output_clone.show();
    });


    window.set_application(Some(app));
    window.show_all();
    image_output.hide();
}