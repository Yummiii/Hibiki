use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use gtk4::{glib::MainContext, prelude::ButtonExt, Button};
use log::debug;

pub fn build_picker() -> Button {
    let picker = Button::builder().icon_name("document-open").build();

    picker.connect_clicked(|_| {
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
                    debug!("File: {}", uri.as_str());
                }
            }
        });
    });

    picker
}
