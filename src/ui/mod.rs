use footer::build_footer;
use gtk4::{
    prelude::{ApplicationExt, BoxExt, GtkWindowExt, WidgetExt},
    Box, Label, Orientation,
};
use header::build_header;
use libadwaita::{Application, ApplicationWindow};

mod footer;
mod header;

pub fn create_ui() -> Application {
    let app = Application::builder()
        .application_id("com.zuraaa.Hibiki")
        .build();

    app.connect_activate(|app| {
        let content = Box::new(Orientation::Vertical, 0);

        let body = Box::new(Orientation::Vertical, 0);
        body.append(&Label::new(Some("place holder")));
        body.set_vexpand(true);

        content.append(&build_header());
        content.append(&body);
        content.append(&build_footer());

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
