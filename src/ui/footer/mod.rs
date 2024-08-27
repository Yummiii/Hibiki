use crate::streamer::ArcPipe;
use gtk4::{
    prelude::{BoxExt, EditableExt, ObjectExt},
    ActionBar, Box, Entry, Orientation,
};
use time_bar::build_time_bar;

mod time_bar;

pub fn build_footer(pipeline: ArcPipe) -> ActionBar {
    let footer = ActionBar::new();

    let btn = Box::new(Orientation::Horizontal, 0);
    let vol = Entry::builder().placeholder_text("Volume").build();

    vol.set_text(
        &(pipeline.playbin.property::<f64>("volume") * 100.)
            .ceil()
            .to_string(),
    );

    vol.connect_changed({
        let pipeline = pipeline.clone();
        move |b| {
            println!("{}", b.text());
            if let Ok(vol) = &b.text().parse::<f64>() {
                pipeline.set_volume(vol / 100.);
            }
        }
    });

    btn.append(&build_time_bar(pipeline));
    btn.append(&vol);
    // btn.append(&label);

    footer.set_center_widget(Some(&btn));

    footer
}
