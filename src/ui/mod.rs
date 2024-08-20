use gtk4::{
    prelude::{ApplicationExt, BoxExt, GtkWindowExt},
    Box, Orientation,
};
use header::build_header;
use libadwaita::{Application, ApplicationWindow};

mod header;

pub fn create_ui() -> Application {
    let app = Application::builder()
        .application_id("com.zuraaa.Hibiki")
        .build();

    app.connect_activate(|app| {
        let content = Box::new(Orientation::Vertical, 0);

        content.append(&build_header());

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hibiki")
            .default_width(700)
            .default_height(500)
            .content(&content)
            .build();
        window.present();
    });

    app
}
