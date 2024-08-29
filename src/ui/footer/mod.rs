use crate::streamer::ArcPipe;
use controls::build_controls;
use gtk4::{prelude::BoxExt, ActionBar, Box, Orientation};
use time_bar::build_time_bar;
use volume::build_volume;

mod controls;
mod time_bar;
mod volume;

pub fn build_footer(pipeline: ArcPipe) -> ActionBar {
    let footer = ActionBar::new();

    let btn = Box::new(Orientation::Horizontal, 0);

    btn.append(&build_controls(pipeline.clone()));
    btn.append(&build_time_bar(pipeline.clone()));
    btn.append(&build_volume());

    footer.set_center_widget(Some(&btn));

    footer
}
