use crate::{messenger::Message, streamer::ArcPipe};
use gstreamer::{prelude::ElementExtManual, ClockTime, Fraction};
use gtk4::{
    glib::{property::PropertySet, spawn_future_local, timeout_add_local, ControlFlow},
    prelude::RangeExt,
    Scale,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::Mutex;

pub fn build_time_bar(pipeline: ArcPipe) -> Scale {
    let time_bar = Scale::builder().hexpand(true).build();

    let started = Arc::new(AtomicBool::new(false));
    let fps = Arc::new(Mutex::new(Fraction::new(0, 1)));

    spawn_future_local({
        let pipeline = pipeline.clone();
        let started = started.clone();
        let time_bar = time_bar.clone();
        let fps = fps.clone();

        async move {
            while let Ok(msg) = pipeline.messenger.receiver.recv().await {
                match msg {
                    Message::FpsFound(ffps) => {
                        *fps.lock().await = ffps;
                    }
                    Message::DurationFound(duration) => {
                        started.set(true);
                        time_bar.set_range(0., duration as f64);
                    }
                }
            }
        }
    });

    timeout_add_local(Duration::from_millis(1), {
        let pipeline = pipeline.clone();
        let time_bar = time_bar.clone();
        let fps = fps.clone();

        move || {
            if started.load(Ordering::Relaxed) {
                let fps = *fps.blocking_lock();
                println!("{:?}", fps);

                if let Some(pos) = pipeline.playbin.query_position::<ClockTime>() {
                    // pipeline.playbin.query
                    time_bar.set_value(pos.mseconds() as f64);
                }
            }
            ControlFlow::Continue
        }
    });

    time_bar
}
