use crate::{
    messenger::{macros::on_message, types::MessageType},
    player::commons::{media_controls::MediaStates, Player},
    state::ArcPipe,
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

pub fn build_time_bar(state: ArcPipe<impl Player>) -> Scale {
    let time_bar = Scale::builder().hexpand(true).build();
    time_bar.set_range(0., 100.);

    let duration = Arc::new(AtomicU64::new(0));

    on_message!(state.messenger, MessageType::StateChanged, MediaStates, {
        let duration = duration.clone();
        let state = state.clone();

        move |player_state| {
            if let MediaStates::Playing = *player_state {
                if let Some(dur) = state.player.duration() {
                    println!("Duration: {}", dur);
                    duration.set(dur);
                }
            }
        }
    });

    time_bar.connect_change_value({
        let state = state.clone();
        let duration = duration.clone();
        let time_bar = time_bar.clone();

        move |_, _, value| {
            // if pipeline.playbin.current_state() == State::Null {
            //     return Propagation::Stop;
            // }

            // pipeline.playbin.set_state(State::Paused).unwrap();

            // let duration = duration.load(Ordering::Relaxed);
            // let width = time_bar.width() as f64;
            // let pos = (value / width) * duration as f64;
            // let pos = ClockTime::from_mseconds(pos as u64);

            // //this sometimes freezes the video, but i have no idea why
            // //also todo: make all the player controls a trait in case i want to switch to a different player
            // if let Ok(()) = pipeline
            //     .playbin
            //     .seek_simple(SeekFlags::TRICKMODE | SeekFlags::FLUSH, pos)
            // {
            //     pipeline.playbin.set_state(State::Playing).unwrap();
            // }

            Propagation::Proceed
        }
    });

    timeout_add_local(Duration::from_millis(1), {
        let state = state.clone();
        let time_bar = time_bar.clone();

        move || {
            let duration = duration.load(Ordering::Relaxed);
            if duration > 0 && state.player.state() == MediaStates::Playing {
                // println!("{:?}", state.player.state());

                let width = time_bar.width() as f64;
                time_bar.set_range(0., width);

                if let Some(pos) = state.player.position() {
                    let scale = (width / duration as f64) * pos as f64;
                    // println!("{}", scale);
                    time_bar.set_value(scale);
                }
            }
            ControlFlow::Continue
        }
    });

    time_bar
}
