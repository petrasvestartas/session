pub(super) use crate::command_dock::theme::{fonts, visuals};

/// The bundled label fonts, main font first.
pub(super) const BUNDLED: [&[u8]; 3] = [
    crate::engine::text::FONT_BYTES,
    crate::engine::text::FALLBACK_BYTES,
    crate::engine::text::SYMBOL_BYTES,
];
