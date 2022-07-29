use super::{KeyLayout, KeyRow};
use crate::keys::Key;

impl KeyLayout {
    pub fn gx502_layout() -> Self {
        Self {
            matches: vec!["GX502".into(), "GU502".into()],
            locale: "US".to_string(),
            rows: vec![
                KeyRow::new(
                    0.8,
                    vec![
                        Key::NormalSpacer,
                        Key::FuncSpacer,
                        Key::VolDown,
                        Key::VolUp,
                        Key::MicMute,
                        Key::Rog,
                    ],
                ),
                KeyRow::new(
                    0.8,
                    vec![
                        Key::Esc,
                        Key::FuncSpacer,
                        Key::F1,
                        Key::F2,
                        Key::F3,
                        Key::F4,
                        Key::FuncSpacer, // not sure which key to put here
                        Key::F5,
                        Key::F6,
                        Key::F7,
                        Key::F8,
                        Key::FuncSpacer,
                        Key::F9,
                        Key::F10,
                        Key::F11,
                        Key::F12,
                        Key::RowEndSpacer,
                        Key::Del,
                    ],
                ),
                KeyRow::new(
                    1.0,
                    vec![
                        Key::Tilde,
                        Key::N1,
                        Key::N2,
                        Key::N3,
                        Key::N4,
                        Key::N5,
                        Key::N6,
                        Key::N7,
                        Key::N8,
                        Key::N9,
                        Key::N0,
                        Key::Hyphen,
                        Key::Equals,
                        Key::BkSpc3_1,
                        Key::BkSpc3_2,
                        Key::BkSpc3_3,
                        Key::RowEndSpacer,
                        Key::Home,
                    ],
                ),
                KeyRow::new(
                    1.0,
                    vec![
                        Key::Tab,
                        Key::Q,
                        Key::W,
                        Key::E,
                        Key::R,
                        Key::T,
                        Key::Y,
                        Key::U,
                        Key::I,
                        Key::O,
                        Key::P,
                        Key::LBracket,
                        Key::RBracket,
                        Key::BackSlash,
                        Key::RowEndSpacer,
                        Key::PgUp,
                    ],
                ),
                KeyRow::new(
                    1.0,
                    vec![
                        Key::Caps,
                        Key::A,
                        Key::S,
                        Key::D,
                        Key::F,
                        Key::G,
                        Key::H,
                        Key::J,
                        Key::K,
                        Key::L,
                        Key::SemiColon,
                        Key::Quote,
                        Key::Return3_1,
                        Key::Return3_2,
                        Key::Return3_3,
                        Key::RowEndSpacer,
                        Key::PgDn,
                    ],
                ),
                KeyRow::new(
                    1.0,
                    vec![
                        Key::LShift,
                        Key::Z,
                        Key::X,
                        Key::C,
                        Key::V,
                        Key::B,
                        Key::N,
                        Key::M,
                        Key::Comma,
                        Key::Period,
                        Key::FwdSlash,
                        Key::Rshift3_1,
                        Key::Rshift3_2,
                        Key::Rshift3_3,
                        Key::RowEndSpacer,
                        Key::End,
                    ],
                ),
                KeyRow::new(
                    1.0,
                    vec![
                        Key::LCtrl,
                        Key::LFn,
                        Key::Meta,
                        Key::LAlt,
                        Key::Space5_1,
                        Key::Space5_2,
                        Key::Space5_3,
                        Key::Space5_4,
                        Key::Space5_5,
                        Key::RAlt,
                        Key::PrtSc,
                        Key::RCtrl,
                        Key::ArrowSpacer,
                        Key::Up,
                        Key::ArrowSpacer,
                        Key::RowEndSpacer,
                        Key::RFn,
                    ],
                ),
                KeyRow::new(
                    1.0,
                    vec![
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::ArrowSpacer,
                        Key::Left,
                        Key::Down,
                        Key::Right,
                        Key::ArrowSpacer,
                    ],
                ),
            ],
        }
    }
}
