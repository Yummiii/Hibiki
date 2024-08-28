use crate::{
    messenger::{macros::on_message, types::MessageType},
    streamer::ArcPipe,
};
use controls::build_controls;
use gstreamer::State;
use gtk4::{
    prelude::{BoxExt, EditableExt, ObjectExt},
    ActionBar, Box, Entry, Orientation,
};
use time_bar::build_time_bar;

mod controls;
mod time_bar;

pub fn build_footer(pipeline: ArcPipe) -> ActionBar {
    let footer = ActionBar::new();

    let btn = Box::new(Orientation::Horizontal, 0);
    let vol = Entry::builder().placeholder_text("Volume").build();

    on_message!(pipeline.messenger, MessageType::StateChanged, State, {
        let vol = vol.clone();
        let pipeline = pipeline.clone();

        move |state| {
            if *state == State::Playing {
                vol.set_text(
                    &(pipeline.playbin.property::<f64>("volume") * 100.)
                        .ceil()
                        .to_string(),
                );
            }
        }
    });

    vol.connect_changed({
        let pipeline = pipeline.clone();
        move |b| {
            if let Ok(vol) = &b.text().parse::<f64>() {
                pipeline.set_volume(vol / 100.);
            }
        }
    });

    btn.append(&build_controls(pipeline.clone()));
    btn.append(&build_time_bar(pipeline.clone()));
    btn.append(&vol);

    footer.set_center_widget(Some(&btn));

    footer
}
