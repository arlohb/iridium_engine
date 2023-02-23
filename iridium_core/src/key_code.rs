use iridium_ecs::ui::InspectorUiField;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

/// Represents a key on the keyboard.
///
/// This is kept close to `egui::Key`,
/// just because I like their choice of keys.
/// (It's not overwhelmingly long, like winit's)
#[allow(missing_docs)]
#[derive(Display, EnumString, EnumIter, PartialEq, Eq, Hash, Clone)]
pub enum KeyCode {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Escape,
    Tab,
    Backspace,
    Enter,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    Minus,
    PlusEquals,
    Other(String),
}

impl From<egui::Key> for KeyCode {
    fn from(value: egui::Key) -> Self {
        match value {
            egui::Key::ArrowDown => Self::ArrowDown,
            egui::Key::ArrowLeft => Self::ArrowLeft,
            egui::Key::ArrowRight => Self::ArrowRight,
            egui::Key::ArrowUp => Self::ArrowUp,
            egui::Key::Escape => Self::Escape,
            egui::Key::Tab => Self::Tab,
            egui::Key::Backspace => Self::Backspace,
            egui::Key::Enter => Self::Enter,
            egui::Key::Space => Self::Space,
            egui::Key::Insert => Self::Insert,
            egui::Key::Delete => Self::Delete,
            egui::Key::Home => Self::Home,
            egui::Key::End => Self::End,
            egui::Key::PageUp => Self::PageUp,
            egui::Key::PageDown => Self::PageDown,
            egui::Key::Num0 => Self::Num0,
            egui::Key::Num1 => Self::Num1,
            egui::Key::Num2 => Self::Num2,
            egui::Key::Num3 => Self::Num3,
            egui::Key::Num4 => Self::Num4,
            egui::Key::Num5 => Self::Num5,
            egui::Key::Num6 => Self::Num6,
            egui::Key::Num7 => Self::Num7,
            egui::Key::Num8 => Self::Num8,
            egui::Key::Num9 => Self::Num9,
            egui::Key::A => Self::A,
            egui::Key::B => Self::B,
            egui::Key::C => Self::C,
            egui::Key::D => Self::D,
            egui::Key::E => Self::E,
            egui::Key::F => Self::F,
            egui::Key::G => Self::G,
            egui::Key::H => Self::H,
            egui::Key::I => Self::I,
            egui::Key::J => Self::J,
            egui::Key::K => Self::K,
            egui::Key::L => Self::L,
            egui::Key::M => Self::M,
            egui::Key::N => Self::N,
            egui::Key::O => Self::O,
            egui::Key::P => Self::P,
            egui::Key::Q => Self::Q,
            egui::Key::R => Self::R,
            egui::Key::S => Self::S,
            egui::Key::T => Self::T,
            egui::Key::U => Self::U,
            egui::Key::V => Self::V,
            egui::Key::W => Self::W,
            egui::Key::X => Self::X,
            egui::Key::Y => Self::Y,
            egui::Key::Z => Self::Z,
            egui::Key::F1 => Self::F1,
            egui::Key::F2 => Self::F2,
            egui::Key::F3 => Self::F3,
            egui::Key::F4 => Self::F4,
            egui::Key::F5 => Self::F5,
            egui::Key::F6 => Self::F6,
            egui::Key::F7 => Self::F7,
            egui::Key::F8 => Self::F8,
            egui::Key::F9 => Self::F9,
            egui::Key::F10 => Self::F10,
            egui::Key::F11 => Self::F11,
            egui::Key::F12 => Self::F12,
            egui::Key::F13 => Self::F13,
            egui::Key::F14 => Self::F14,
            egui::Key::F15 => Self::F15,
            egui::Key::F16 => Self::F16,
            egui::Key::F17 => Self::F17,
            egui::Key::F18 => Self::F18,
            egui::Key::F19 => Self::F19,
            egui::Key::F20 => Self::F20,
            egui::Key::Minus => Self::Minus,
            egui::Key::PlusEquals => Self::PlusEquals,
        }
    }
}

impl From<winit::event::VirtualKeyCode> for KeyCode {
    #[allow(clippy::too_many_lines)]
    fn from(value: winit::event::VirtualKeyCode) -> Self {
        match value {
            winit::event::VirtualKeyCode::Key1 | winit::event::VirtualKeyCode::Numpad1 => {
                Self::Num1
            }
            winit::event::VirtualKeyCode::Key2 | winit::event::VirtualKeyCode::Numpad2 => {
                Self::Num2
            }
            winit::event::VirtualKeyCode::Key3 | winit::event::VirtualKeyCode::Numpad3 => {
                Self::Num3
            }
            winit::event::VirtualKeyCode::Key4 | winit::event::VirtualKeyCode::Numpad4 => {
                Self::Num4
            }
            winit::event::VirtualKeyCode::Key5 | winit::event::VirtualKeyCode::Numpad5 => {
                Self::Num5
            }
            winit::event::VirtualKeyCode::Key6 | winit::event::VirtualKeyCode::Numpad6 => {
                Self::Num6
            }
            winit::event::VirtualKeyCode::Key7 | winit::event::VirtualKeyCode::Numpad7 => {
                Self::Num7
            }
            winit::event::VirtualKeyCode::Key8 | winit::event::VirtualKeyCode::Numpad8 => {
                Self::Num8
            }
            winit::event::VirtualKeyCode::Key9 | winit::event::VirtualKeyCode::Numpad9 => {
                Self::Num9
            }
            winit::event::VirtualKeyCode::Key0 | winit::event::VirtualKeyCode::Numpad0 => {
                Self::Num0
            }
            winit::event::VirtualKeyCode::A => Self::A,
            winit::event::VirtualKeyCode::B => Self::B,
            winit::event::VirtualKeyCode::C => Self::C,
            winit::event::VirtualKeyCode::D => Self::D,
            winit::event::VirtualKeyCode::E => Self::E,
            winit::event::VirtualKeyCode::F => Self::F,
            winit::event::VirtualKeyCode::G => Self::G,
            winit::event::VirtualKeyCode::H => Self::H,
            winit::event::VirtualKeyCode::I => Self::I,
            winit::event::VirtualKeyCode::J => Self::J,
            winit::event::VirtualKeyCode::K => Self::K,
            winit::event::VirtualKeyCode::L => Self::L,
            winit::event::VirtualKeyCode::M => Self::M,
            winit::event::VirtualKeyCode::N => Self::N,
            winit::event::VirtualKeyCode::O => Self::O,
            winit::event::VirtualKeyCode::P => Self::P,
            winit::event::VirtualKeyCode::Q => Self::Q,
            winit::event::VirtualKeyCode::R => Self::R,
            winit::event::VirtualKeyCode::S => Self::S,
            winit::event::VirtualKeyCode::T => Self::T,
            winit::event::VirtualKeyCode::U => Self::U,
            winit::event::VirtualKeyCode::V => Self::V,
            winit::event::VirtualKeyCode::W => Self::W,
            winit::event::VirtualKeyCode::X => Self::X,
            winit::event::VirtualKeyCode::Y => Self::Y,
            winit::event::VirtualKeyCode::Z => Self::Z,
            winit::event::VirtualKeyCode::Escape => Self::Escape,
            winit::event::VirtualKeyCode::Back => Self::Backspace,
            winit::event::VirtualKeyCode::F1 => Self::F1,
            winit::event::VirtualKeyCode::F2 => Self::F2,
            winit::event::VirtualKeyCode::F3 => Self::F3,
            winit::event::VirtualKeyCode::F4 => Self::F4,
            winit::event::VirtualKeyCode::F5 => Self::F5,
            winit::event::VirtualKeyCode::F6 => Self::F6,
            winit::event::VirtualKeyCode::F7 => Self::F7,
            winit::event::VirtualKeyCode::F8 => Self::F8,
            winit::event::VirtualKeyCode::F9 => Self::F9,
            winit::event::VirtualKeyCode::F10 => Self::F10,
            winit::event::VirtualKeyCode::F11 => Self::F11,
            winit::event::VirtualKeyCode::F12 => Self::F12,
            winit::event::VirtualKeyCode::F13 => Self::F13,
            winit::event::VirtualKeyCode::F14 => Self::F14,
            winit::event::VirtualKeyCode::F15 => Self::F15,
            winit::event::VirtualKeyCode::F16 => Self::F16,
            winit::event::VirtualKeyCode::F17 => Self::F17,
            winit::event::VirtualKeyCode::F18 => Self::F18,
            winit::event::VirtualKeyCode::F19 => Self::F19,
            winit::event::VirtualKeyCode::F20 => Self::F20,
            winit::event::VirtualKeyCode::F21 => Self::Other("F21".to_string()),
            winit::event::VirtualKeyCode::F22 => Self::Other("F22".to_string()),
            winit::event::VirtualKeyCode::F23 => Self::Other("F23".to_string()),
            winit::event::VirtualKeyCode::F24 => Self::Other("F24".to_string()),
            winit::event::VirtualKeyCode::Snapshot => Self::Other("Snapshot".to_string()),
            winit::event::VirtualKeyCode::Scroll => Self::Other("Scroll".to_string()),
            winit::event::VirtualKeyCode::Pause => Self::Other("Pause".to_string()),
            winit::event::VirtualKeyCode::Insert => Self::Insert,
            winit::event::VirtualKeyCode::Home => Self::Home,
            winit::event::VirtualKeyCode::Delete => Self::Delete,
            winit::event::VirtualKeyCode::End => Self::End,
            winit::event::VirtualKeyCode::PageDown => Self::PageDown,
            winit::event::VirtualKeyCode::PageUp => Self::PageUp,
            winit::event::VirtualKeyCode::Left => Self::ArrowLeft,
            winit::event::VirtualKeyCode::Up => Self::ArrowUp,
            winit::event::VirtualKeyCode::Right => Self::ArrowRight,
            winit::event::VirtualKeyCode::Down => Self::ArrowDown,
            winit::event::VirtualKeyCode::Return => Self::Enter,
            winit::event::VirtualKeyCode::Space => Self::Space,
            winit::event::VirtualKeyCode::Compose => Self::Other("Compose".to_string()),
            winit::event::VirtualKeyCode::Caret => Self::Other("Caret".to_string()),
            winit::event::VirtualKeyCode::Numlock => Self::Other("Numlock".to_string()),
            winit::event::VirtualKeyCode::NumpadAdd => Self::Other("NumpadAdd".to_string()),
            winit::event::VirtualKeyCode::NumpadDivide => Self::Other("NumpadDivide".to_string()),
            winit::event::VirtualKeyCode::NumpadDecimal => Self::Other("NumpadDecimal".to_string()),
            winit::event::VirtualKeyCode::NumpadComma => Self::Other("NumpadComma".to_string()),
            winit::event::VirtualKeyCode::NumpadEnter => Self::Other("NumpadEnter".to_string()),
            winit::event::VirtualKeyCode::NumpadEquals => Self::Other("NumpadEquals".to_string()),
            winit::event::VirtualKeyCode::NumpadMultiply => {
                Self::Other("NumpadMultiply".to_string())
            }
            winit::event::VirtualKeyCode::NumpadSubtract => {
                Self::Other("NumpadSubtract".to_string())
            }
            winit::event::VirtualKeyCode::AbntC1 => Self::Other("AbntC1".to_string()),
            winit::event::VirtualKeyCode::AbntC2 => Self::Other("AbntC2".to_string()),
            winit::event::VirtualKeyCode::Apostrophe => Self::Other("Apostrophe".to_string()),
            winit::event::VirtualKeyCode::Apps => Self::Other("Apps".to_string()),
            winit::event::VirtualKeyCode::Asterisk => Self::Other("Asterisk".to_string()),
            winit::event::VirtualKeyCode::At => Self::Other("At".to_string()),
            winit::event::VirtualKeyCode::Ax => Self::Other("Ax".to_string()),
            winit::event::VirtualKeyCode::Backslash => Self::Other("Backslash".to_string()),
            winit::event::VirtualKeyCode::Calculator => Self::Other("Calculator".to_string()),
            winit::event::VirtualKeyCode::Capital => Self::Other("Capital".to_string()),
            winit::event::VirtualKeyCode::Colon => Self::Other("Colon".to_string()),
            winit::event::VirtualKeyCode::Comma => Self::Other("Comma".to_string()),
            winit::event::VirtualKeyCode::Convert => Self::Other("Convert".to_string()),
            winit::event::VirtualKeyCode::Equals => Self::PlusEquals,
            winit::event::VirtualKeyCode::Grave => Self::Other("Grave".to_string()),
            winit::event::VirtualKeyCode::Kana => Self::Other("Kana".to_string()),
            winit::event::VirtualKeyCode::Kanji => Self::Other("Kanji".to_string()),
            winit::event::VirtualKeyCode::LAlt => Self::Other("LAlt".to_string()),
            winit::event::VirtualKeyCode::LBracket => Self::Other("LBracket".to_string()),
            winit::event::VirtualKeyCode::LControl => Self::Other("LControl".to_string()),
            winit::event::VirtualKeyCode::LShift => Self::Other("LShift".to_string()),
            winit::event::VirtualKeyCode::LWin => Self::Other("LWin".to_string()),
            winit::event::VirtualKeyCode::Mail => Self::Other("Mail".to_string()),
            winit::event::VirtualKeyCode::MediaSelect => Self::Other("MediaSelect".to_string()),
            winit::event::VirtualKeyCode::MediaStop => Self::Other("MediaStop".to_string()),
            winit::event::VirtualKeyCode::Minus => Self::Minus,
            winit::event::VirtualKeyCode::Mute => Self::Other("Mute".to_string()),
            winit::event::VirtualKeyCode::MyComputer => Self::Other("MyComputer".to_string()),
            winit::event::VirtualKeyCode::NavigateForward => {
                Self::Other("NavigateForward".to_string())
            }
            winit::event::VirtualKeyCode::NavigateBackward => {
                Self::Other("NavigateBackward".to_string())
            }
            winit::event::VirtualKeyCode::NextTrack => Self::Other("NextTrack".to_string()),
            winit::event::VirtualKeyCode::NoConvert => Self::Other("NoConvert".to_string()),
            winit::event::VirtualKeyCode::OEM102 => Self::Other("OEM102".to_string()),
            winit::event::VirtualKeyCode::Period => Self::Other("Period".to_string()),
            winit::event::VirtualKeyCode::PlayPause => Self::Other("PlayPause".to_string()),
            winit::event::VirtualKeyCode::Plus => Self::Other("Plus".to_string()),
            winit::event::VirtualKeyCode::Power => Self::Other("Power".to_string()),
            winit::event::VirtualKeyCode::PrevTrack => Self::Other("PrevTrack".to_string()),
            winit::event::VirtualKeyCode::RAlt => Self::Other("RAlt".to_string()),
            winit::event::VirtualKeyCode::RBracket => Self::Other("RBracket".to_string()),
            winit::event::VirtualKeyCode::RControl => Self::Other("RControl".to_string()),
            winit::event::VirtualKeyCode::RShift => Self::Other("RShift".to_string()),
            winit::event::VirtualKeyCode::RWin => Self::Other("RWin".to_string()),
            winit::event::VirtualKeyCode::Semicolon => Self::Other("Semicolon".to_string()),
            winit::event::VirtualKeyCode::Slash => Self::Other("Slash".to_string()),
            winit::event::VirtualKeyCode::Sleep => Self::Other("Sleep".to_string()),
            winit::event::VirtualKeyCode::Stop => Self::Other("Stop".to_string()),
            winit::event::VirtualKeyCode::Sysrq => Self::Other("Sysrq".to_string()),
            winit::event::VirtualKeyCode::Tab => Self::Other("Tab".to_string()),
            winit::event::VirtualKeyCode::Underline => Self::Other("Underline".to_string()),
            winit::event::VirtualKeyCode::Unlabeled => Self::Other("Unlabeled".to_string()),
            winit::event::VirtualKeyCode::VolumeDown => Self::Other("VolumeDown".to_string()),
            winit::event::VirtualKeyCode::VolumeUp => Self::Other("VolumeUp".to_string()),
            winit::event::VirtualKeyCode::Wake => Self::Other("Wake".to_string()),
            winit::event::VirtualKeyCode::WebBack => Self::Other("WebBack".to_string()),
            winit::event::VirtualKeyCode::WebFavorites => Self::Other("WebFavorites".to_string()),
            winit::event::VirtualKeyCode::WebForward => Self::Other("WebForward".to_string()),
            winit::event::VirtualKeyCode::WebHome => Self::Other("WebHome".to_string()),
            winit::event::VirtualKeyCode::WebRefresh => Self::Other("WebRefresh".to_string()),
            winit::event::VirtualKeyCode::WebSearch => Self::Other("WebSearch".to_string()),
            winit::event::VirtualKeyCode::WebStop => Self::Other("WebStop".to_string()),
            winit::event::VirtualKeyCode::Yen => Self::Other("Yen".to_string()),
            winit::event::VirtualKeyCode::Copy => Self::Other("Copy".to_string()),
            winit::event::VirtualKeyCode::Paste => Self::Other("Paste".to_string()),
            winit::event::VirtualKeyCode::Cut => Self::Other("Cut".to_string()),
        }
    }
}

impl InspectorUiField for KeyCode {
    fn ui(&mut self, ui: &mut egui::Ui, attributes: iridium_ecs::ui::InspectorUiFieldAttributes) {
        let id = attributes.get::<i32>("id").unwrap_or(0);

        egui::ComboBox::from_id_source(format!("Key selector {id}"))
            .selected_text(self.to_string())
            .show_ui(ui, |ui| {
                for key_code in Self::iter() {
                    ui.selectable_value(self, key_code.clone(), key_code.to_string());
                }
            });
    }
}
