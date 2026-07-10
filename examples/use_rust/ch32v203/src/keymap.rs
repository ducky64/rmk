use rmk::types::action::KeyAction;
use rmk::{a, k, layer};
pub(crate) const COL: usize = 3;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 1;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(AudioVolUp), k!(B), k!(AudioVolDown)],
            [k!(AudioVolUp), k!(B), k!(AudioVolDown)],
            [k!(AudioVolUp), k!(B), k!(AudioVolDown)],
            [k!(AudioVolUp), k!(B), k!(AudioVolDown)]
        ]),
    ]
}
