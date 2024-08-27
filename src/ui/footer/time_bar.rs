use crate::{messenger::MessageType, streamer::ArcPipe};
use gstreamer::{
    prelude::{ElementExt, ElementExtManual},
    ClockTime, SeekFlags, State,
};
use gtk4::{
    glib::{property::PropertySet, timeout_add_local, ControlFlow, Propagation},
    prelude::{RangeExt, WidgetExt},
    Scale,
};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

pub fn build_time_bar(pipeline: ArcPipe) -> Scale {
    let time_bar = Scale::builder().hexpand(true).build();
    time_bar.set_range(0., 1000.);

    let duration = Arc::new(AtomicU64::new(0));

    {
        let duration = duration.clone();
        pipeline
            .messenger
            .on_message(MessageType::DurationFound, move |new_duration: Box<u64>| {
                duration.set(*new_duration);
            });
    }

    time_bar.connect_change_value({
        let pipeline = pipeline.clone();
        let duration = duration.clone();
        let time_bar = time_bar.clone();

        move |_, _, _value| {
            pipeline.playbin.set_state(State::Paused).unwrap();

            let duration = duration.load(Ordering::Relaxed);
            let width = time_bar.width() as f64;
            let pos = (time_bar.value() / width) * duration as f64;
            let pos = ClockTime::from_mseconds(pos as u64);

            println!("Pos: {}", pos);
            println!("Duartion: {}", ClockTime::from_mseconds(duration));

            //this sometimes freezes the video, but i have no idea why
            if let Ok(()) = pipeline
                .playbin
                .seek_simple(SeekFlags::TRICKMODE | SeekFlags::FLUSH, pos)
            {
                pipeline.playbin.set_state(State::Playing).unwrap();
            }

            Propagation::Proceed
        }
    });

    timeout_add_local(Duration::from_millis(1), {
        let pipeline = pipeline.clone();
        let time_bar = time_bar.clone();

        move || {
            let duration = duration.load(Ordering::Relaxed);
            if duration > 0 && pipeline.playbin.current_state() == State::Playing {
                // println!("{:?}", pipeline.playbin.current_state());

                let width = time_bar.width() as f64;
                time_bar.set_range(0., width);

                if let Some(pos) = pipeline.playbin.query_position::<ClockTime>() {
                    let scale = (width / duration as f64) * pos.mseconds() as f64;
                    // println!("{}", scale);
                    time_bar.set_value(scale);
                }
            }
            ControlFlow::Continue
        }
    });

    time_bar
}
