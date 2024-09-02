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
                    duration.set(dur);
                }
            }
        }
    });

    // on_message!(state.messenger, MessageType::Eos, (), {
    //     let time_bar = time_bar.clone();

    //     move |_| {
    //         // time_bar.set_value(0.);
    //     }
    // });

    time_bar.connect_change_value({
        let state = state.clone();
        let duration = duration.clone();
        let time_bar = time_bar.clone();

        move |_, _, value| {
            if state.player.state() == MediaStates::Stopped {
                return Propagation::Stop;
            }

            let duration = duration.load(Ordering::Relaxed);
            let width = time_bar.width() as f64;
            let pos = (value / width) * duration as f64;

            //this sometimes freezes the video, but i have no idea why
            if let Ok(()) = state.player.seek(pos as u64) {
                Propagation::Proceed
            } else {
                Propagation::Stop
            }
        }
    });

    timeout_add_local(Duration::from_millis(1), {
        let state = state.clone();
        let time_bar = time_bar.clone();

        move || {
            let duration = duration.load(Ordering::Relaxed);
            if duration > 0 && state.player.state() == MediaStates::Playing {
                let width = time_bar.width() as f64;
                time_bar.set_range(0., width);

                if let Some(pos) = state.player.position() {
                    let scale = (width / duration as f64) * pos as f64;
                    time_bar.set_value(scale);
                }
            }
            ControlFlow::Continue
        }
    });

    time_bar
}
