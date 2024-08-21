use crate::streamer::ArcPipe;
use gtk4::{
    prelude::{BoxExt, EditableExt, ObjectExt},
    ActionBar, Box, Entry, Orientation,
};

pub fn build_footer(pipeline: ArcPipe) -> ActionBar {
    let footer = ActionBar::new();

    let btn = Box::new(Orientation::Horizontal, 0);

    let vol = Entry::builder().placeholder_text("Volume").build();
    vol.set_text(
        &(pipeline.audio.volume.property::<f64>("volume") * 100.)
            .ceil()
            .to_string(),
    );

    vol.connect_changed(move |b| {
        println!("{}", b.text());
        if let Ok(vol) = &b.text().parse::<f64>() {
            pipeline.audio.set_volume(vol / 100.);
        }
    });

    btn.append(&vol);

    footer.set_center_widget(Some(&btn));

    footer
}
