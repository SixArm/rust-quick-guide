use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};

fn main() -> glib::ExitCode {

    // Create an application (with builder)
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {

        // Create a label
        let label = Label::builder()
            .label("My Label")
            .margin_top(8)
            .margin_bottom(8)
            .build();

        // Create a button
        let button = Button::builder()
            .label("My Button")
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .build();

        // Connect to the button's "clicked" signal
        button.connect_clicked(|_button| {
            eprintln!("Clicked!");
        });

        // Create a vertical box for layout
        let vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .build();

        // Add the label and button to the vertical box
        vbox.append(&label);
        vbox.append(&button);

        // Create a window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(300)
            .default_height(200)
            .title("My Window")
            .child(&vbox)
            .build();

        // Show the window
        window.present();
    });

    // Run the application GTK event loop
    app.run() 

}

