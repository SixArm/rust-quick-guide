# egui crate for pure Rust GUIs

<https://crates.io/crates/egui>

The Rust egui crate is easy-to-use, lightweight solution for creating graphical user interfaces (GUIs) using pure Rust. The crate uses OpenGL for rendering, which enables it to offer high performance and low resource usage. It also supports a wide range of widgets, such as buttons, text inputs, sliders, and checkboxes, making it a versatile option for creating various types of user interfaces.

One of the key features of the Rust egui crate is its ability to run completely in a web browser without needing any server-side code. This makes it ideal for creating browser applications or web-based dashboards. Additionally, it offers a range of customization options, including the ability to change colors, fonts, and layout of the user interface.

Example excerpt of egui:

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Hello, World!");
    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name)
            .labelled_by(name_label.id);
    });
    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
    ui.label(format!("name '{}', age {}", self.name, self.age));
});
```

The Rust eframe crate is the official egui framework crate. If you are planning to write an app for web or native, and want to use egui for everything, then eframe is for you.

Example excerpt of eframe:

```
let options = eframe::NativeOptions {
    initial_window_size: Some(egui::vec2(320.0, 240.0)),
    ..Default::default()
};
eframe::run_native(
    "My App",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
)
```
