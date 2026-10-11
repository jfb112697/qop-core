use serde::{Deserialize, Serialize};

// Physical key identifiers, named after the W3C UI Events `code` values (the same names winit uses).
// `Other` covers any digital input not listed here: unlisted keys, gamepad buttons, MIDI notes, etc.
// Front ends convert their own key type to this one at the edge.
macro_rules! keys {
    [ $( $name:ident ),+ $(,)? ] => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Key {
            $( $name, )+
            Other(u32),
        }

        #[cfg(feature = "winit")]
        impl Key {
            pub fn from_winit(key_code: winit::keyboard::KeyCode) -> Option<Key> {
                match key_code {
                    $( winit::keyboard::KeyCode::$name => Some(Key::$name), )+
                    _ => None,
                }
            }
        }
    };
}

keys![
    Backquote, Backslash, BracketLeft, BracketRight, Comma,
    Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9,
    Equal, IntlBackslash, IntlRo, IntlYen,
    KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM,
    KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
    Minus, Period, Quote, Semicolon, Slash,
    AltLeft, AltRight, Backspace, CapsLock, ContextMenu, ControlLeft, ControlRight,
    Enter, SuperLeft, SuperRight, ShiftLeft, ShiftRight, Space, Tab,
    Delete, End, Home, Insert, PageDown, PageUp,
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp,
    NumLock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadDecimal, NumpadDivide, NumpadEnter, NumpadEqual, NumpadMultiply, NumpadSubtract,
    Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    PrintScreen, ScrollLock, Pause,
];
