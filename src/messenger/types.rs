#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MessageType {
    StateChanged,
    Eos,
    SubtitlesFound,
}
