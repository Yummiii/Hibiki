use crate::streamer::ArcPipe;
use gtk4::{
    glib::spawn_future_local,
    prelude::{BoxExt, EditableExt, ObjectExt},
    ActionBar, Box, Entry, Label, Orientation,
};

pub fn build_footer(pipeline: ArcPipe) -> ActionBar {
    let footer = ActionBar::new();

    let btn = Box::new(Orientation::Horizontal, 0);

    let vol = Entry::builder().placeholder_text("Volume").build();
    let label = Label::new(Some("kabel"));

    vol.set_text(
        &(pipeline.playbin.property::<f64>("volume") * 100.)
            .ceil()
            .to_string(),
    );

    spawn_future_local({
        let pipeline = pipeline.clone();
        let label = label.clone();

        async move {
            while let Ok(msg) = pipeline.receiver.recv().await {
                label.set_text(&msg);
            }
        }
    });

    vol.connect_changed(move |b| {
        println!("{}", b.text());
        if let Ok(vol) = &b.text().parse::<f64>() {
            pipeline.set_volume(vol / 100.);
        }
    });

    btn.append(&vol);
    btn.append(&label);

    footer.set_center_widget(Some(&btn));

    footer
}
