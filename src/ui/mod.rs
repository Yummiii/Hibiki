use crate::streamer::ArcPipe;
use footer::build_footer;
use gtk4::{
    gdk::Paintable,
    prelude::{ApplicationExt, BoxExt, GtkWindowExt, ObjectExt, WidgetExt},
    Box, Orientation, Picture,
};
use header::build_header;
use libadwaita::{Application, ApplicationWindow};

mod footer;
mod header;

pub fn create_ui(pipeline: ArcPipe) -> Application {
    let app = Application::builder()
        .application_id("com.zuraaa.Hibiki")
        .build();

    app.connect_activate(move |app| {
        let content = Box::new(Orientation::Vertical, 0);

        let body = Box::new(Orientation::Vertical, 0);
        // body.append(&Picture::for_filename(
        //     "/home/yummi/Downloads/__hibiki_kantai_collection_drawn_by_kashimu__78b0dddec511252cfe47f242cc3649b9.png",
        // ));

        let video = Picture::new();
        video.set_paintable(Some(
            &pipeline.video.widget.property::<Paintable>("paintable"),
        ));
        body.append(&video);

        body.set_vexpand(true);

        content.append(&build_header(pipeline.clone()));
        content.append(&body);
        content.append(&build_footer(pipeline.clone()));

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
