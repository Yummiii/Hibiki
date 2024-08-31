use crate::{
    messenger::{macros::on_message, types::MessageType},
    streamer::ArcPipe,
};
use footer::build_footer;
use gstreamer::State;
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
        let video = Picture::new();

        // video.set_visible(false);
        video.set_paintable(Some(&pipeline.widget.property::<Paintable>("paintable")));
        video.set_vexpand(true);

        on_message!(pipeline.messenger, MessageType::StateChanged, State, {
            let video = video.clone();
            move |state| {
                if *state == State::Playing && !video.is_visible() {
                    video.set_visible(true);
                }
            }
        });

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
