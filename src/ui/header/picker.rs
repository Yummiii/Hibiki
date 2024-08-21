use crate::streamer::ArcPipe;
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use gstreamer::{prelude::ElementExt, State};
use gtk4::{
    glib::MainContext,
    prelude::{ButtonExt, ObjectExt},
    Button,
};
use log::{debug, error};

pub fn build_picker(pipeline: ArcPipe) -> Button {
    let picker = Button::builder().icon_name("document-open").build();
    picker.connect_clicked(move |_| {
        let pipeline = pipeline.clone();

        MainContext::default().spawn_local(async move {
            //todo: make this right
            let selected = SelectedFiles::open_file()
                .title("Select a video")
                .accept_label("Select")
                .modal(false)
                .multiple(false)
                .filter(FileFilter::new("Videos").mimetype("video/*"))
                .send()
                .await
                .unwrap();

            if let Ok(files) = selected.response() {
                if let Some(uri) = files.uris().iter().next() {
                    let uri = uri.as_str();
                    debug!("File: {}", uri);

                    pipeline.source.set_property("uri", uri);

                    pipeline.pipeline.set_state(State::Null).unwrap();
                    if let Err(err) = pipeline.pipeline.set_state(State::Playing) {
                        error!("Failed to start pipeline: {}", err);
                    }
                }
            }
        });
    });

    picker
}
