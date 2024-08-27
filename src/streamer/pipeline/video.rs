// use crate::streamer::ArcPipe;
// use gtk4::glib::spawn_future_local;

// pub trait VideoTraits {
//     fn state_changed<F: Fn(String) + 'static>(&self, f: F);
// }

// impl VideoTraits for ArcPipe {
//     fn state_changed<F: Fn(String) + 'static>(&self, f: F) {
//         spawn_future_local({
//             let pipeline = self.clone();
//             async move {
//                 while let Ok(msg) = pipeline.receiver.recv().await {
//                     f(msg);
//                 }
//             }
//         });
//     }
// }
