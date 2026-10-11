use serde::{Deserialize, Serialize};

// Digital (on/off) input identifiers. Front ends convert their own event types to these at the edge.
// Keyboard keys are named after the W3C UI Events `code` values (the same names winit uses).
// `Other` covers any digital input not listed here.
macro_rules! keys {
    (keyboard [ $( $name:ident ),+ $(,)? ] other [ $( $extra:ident ),+ $(,)? ]) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Key {
            $( $name, )+
            $( $extra, )+
            MidiNote(u8),
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

keys!(
    keyboard [
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
    ]
    // Gamepad buttons, named by position (XInput A/B/X/Y are South/East/West/North).
    // PadLeftTrigger / PadRightTrigger are the triggers treated as buttons; an AnalogTrigger can press them with a velocity.
    other [
        PadSouth, PadEast, PadWest, PadNorth,
        PadLeftShoulder, PadRightShoulder, PadLeftTrigger, PadRightTrigger,
        PadLeftThumb, PadRightThumb, PadStart, PadBack,
        PadDpadUp, PadDpadDown, PadDpadLeft, PadDpadRight,
    ]
);

// Continuous input identifiers. Values are sent already normalized by the front end:
// 0.0..=1.0 for triggers, CCs and pressure, -1.0..=1.0 for stick axes and pitch bend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Analog {
    PadLeftStickX,
    PadLeftStickY,
    PadRightStickX,
    PadRightStickY,
    PadLeftTrigger,
    PadRightTrigger,
    MidiCc(u8),
    MidiPitchBend,
    MidiChannelPressure,
    Other(u32),
}
