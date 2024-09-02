use crate::{player::commons::Player, state::ArcPipe};
use footer::build_footer;
use gtk4::{
    prelude::{ApplicationExt, BoxExt, GtkWindowExt, WidgetExt},
    Box, Orientation, Picture,
};
use header::build_header;
use libadwaita::{Application, ApplicationWindow};

mod footer;
mod header;

pub fn create_ui(state: ArcPipe<impl Player>) -> Application {
    let app = Application::builder()
        .application_id("com.zuraaa.Hibiki")
        .build();

    app.connect_activate(move |app| {
        let content = Box::new(Orientation::Vertical, 0);

        let body = Box::new(Orientation::Vertical, 0);
        let video = Picture::new();

        state.player.set_widget(&video);

        video.set_vexpand(true);

        // on_message!(state.messenger, MessageType::StateChanged, MediaStates, {
        //     let video = video.clone();
        //     move |state| {
        //         if *state == MediaStates::Playing && !video.is_visible() {
        //             video.set_visible(true);
        //         }
        //     }
        // });

        body.append(&video);
        body.set_vexpand(true);

        content.append(&build_header(state.clone()));
        content.append(&body);
        content.append(&build_footer(state.clone()));

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
