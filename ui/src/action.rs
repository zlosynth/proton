#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Action {
    EncoderDown,
    EncoderUp,
    EncoderClick,
}
