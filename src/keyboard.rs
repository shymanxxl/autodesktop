#![allow(unused)]
use libc::{c_int, c_uchar, wchar_t};
use std::thread::sleep;
use std::time::Duration;
pub struct KeyBoard {
    paste_shortcut_key: [u8; 3],
}
#[link(name = "low")]
extern "C" {
    fn Key_press(key: c_int);
    fn Key_release(key: c_int);
    fn Key_text(s: *const wchar_t, a: *const c_uchar);

}

impl KeyBoard {
    pub const EMPTY: u8 = 0x00;
    pub const BACKSPACE: u8 = 0x08;
    pub const TAB: u8 = 0x09;
    pub const CLEAR: u8 = 0x0C;
    pub const ENTER: u8 = 0x0D;
    pub const SHIFT: u8 = 0x10;
    pub const CTRL: u8 = 0x11;
    pub const ALT: u8 = 0x12;
    pub const PAUSE: u8 = 0x13;
    pub const KANA: u8 = 0x14;
    pub const HANGUL: u8 = 0x15;
    pub const IME_ON: u8 = 0x16;
    pub const JUNJA: u8 = 0x17;
    pub const FINAL: u8 = 0x18;
    pub const HANJA: u8 = 0x19;
    pub const KANJI: u8 = 0x19;
    pub const IME_OFF: u8 = 0x1A;
    pub const ESC: u8 = 0x1B;
    pub const CONVERT: u8 = 0x1C;
    pub const NONCONVERT: u8 = 0x1D;
    pub const ACCEPT: u8 = 0x1E;
    pub const MODECHANGE: u8 = 0x1F;
    pub const SAPCE: u8 = 0x20;
    pub const PAGE_UP: u8 = 0x21;
    pub const PAGE_DOWN: u8 = 0x22;
    pub const END: u8 = 0x23;
    pub const HOME: u8 = 0x24;
    pub const LEFT: u8 = 0x25;
    pub const UP: u8 = 0x26;
    pub const RIGHT: u8 = 0x27;
    pub const DOWN: u8 = 0x28;
    pub const SELECT: u8 = 0x29;
    pub const PRINT: u8 = 0x2A;
    pub const EXECUTE: u8 = 0x2B;
    pub const SNAPSHOT: u8 = 0x2C;
    pub const INS: u8 = 0x2D;
    pub const DEL: u8 = 0x2E;
    pub const HELP: u8 = 0x2F;
    pub const ZERO: u8 = 0x30;
    pub const ONE: u8 = 0x31;
    pub const TWO: u8 = 0x32;
    pub const THREE: u8 = 0x33;
    pub const FOUR: u8 = 0x34;
    pub const FIVE: u8 = 0x35;
    pub const SIX: u8 = 0x36;
    pub const SEVEN: u8 = 0x37;
    pub const EIGHT: u8 = 0x38;
    pub const NINE: u8 = 0x39;
    pub const A: u8 = 0x41;
    pub const B: u8 = 0x42;
    pub const C: u8 = 0x43;
    pub const D: u8 = 0x44;
    pub const E: u8 = 0x45;
    pub const F: u8 = 0x46;
    pub const G: u8 = 0x47;
    pub const H: u8 = 0x48;
    pub const I: u8 = 0x49;
    pub const J: u8 = 0x4A;
    pub const K: u8 = 0x4B;
    pub const L: u8 = 0x4C;
    pub const M: u8 = 0x4D;
    pub const N: u8 = 0x4E;
    pub const O: u8 = 0x4F;
    pub const P: u8 = 0x50;
    pub const Q: u8 = 0x51;
    pub const R: u8 = 0x52;
    pub const S: u8 = 0x53;
    pub const T: u8 = 0x54;
    pub const U: u8 = 0x55;
    pub const V: u8 = 0x56;
    pub const W: u8 = 0x57;
    pub const X: u8 = 0x58;
    pub const Y: u8 = 0x59;
    pub const Z: u8 = 0x5A;
    pub const LWIN: u8 = 0x5B;
    pub const RWIN: u8 = 0x5C;
    pub const APPS: u8 = 0x5D;
    pub const SLEEP: u8 = 0x5F;
    pub const NUMPAD0: u8 = 0x60;
    pub const NUMPAD1: u8 = 0x61;
    pub const NUMPAD2: u8 = 0x62;
    pub const NUMPAD3: u8 = 0x63;
    pub const NUMPAD4: u8 = 0x64;
    pub const NUMPAD5: u8 = 0x65;
    pub const NUMPAD6: u8 = 0x66;
    pub const NUMPAD7: u8 = 0x67;
    pub const NUMPAD8: u8 = 0x68;
    pub const NUMPAD9: u8 = 0x69;
    pub const MULTIPLY: u8 = 0x6A;
    pub const ADD: u8 = 0x6B;
    pub const SEPARATOP: u8 = 0x6C;
    pub const SUB: u8 = 0x6D;
    pub const DECIMAL: u8 = 0x6E;
    pub const DIV: u8 = 0x6F;
    pub const F1: u8 = 0x70;
    pub const F2: u8 = 0x71;
    pub const F3: u8 = 0x72;
    pub const F4: u8 = 0x73;
    pub const F5: u8 = 0x74;
    pub const F6: u8 = 0x75;
    pub const F7: u8 = 0x76;
    pub const F8: u8 = 0x77;
    pub const F9: u8 = 0x78;
    pub const F10: u8 = 0x79;
    pub const F11: u8 = 0x7A;
    pub const F12: u8 = 0x7B;
    pub const F13: u8 = 0x7C;
    pub const F14: u8 = 0x7D;
    pub const F15: u8 = 0x7E;
    pub const F16: u8 = 0x7F;
    pub const F17: u8 = 0x80;
    pub const F18: u8 = 0x81;
    pub const F19: u8 = 0x82;
    pub const F20: u8 = 0x83;
    pub const F21: u8 = 0x84;
    pub const F22: u8 = 0x85;
    pub const F23: u8 = 0x86;
    pub const F24: u8 = 0x87;
    pub const NUMLOCK: u8 = 0x90;
    pub const SCROLL_LOCK: u8 = 0x91;
    pub const LSHIFT: u8 = 0xA0;
    pub const RSHIFT: u8 = 0xA1;
    pub const LCTRL: u8 = 0xA2;
    pub const RCTRL: u8 = 0xA3;
    pub const LALT: u8 = 0xA4;
    pub const RALT: u8 = 0xA5;
    pub const BROWSER_BACK: u8 = 0xA6;
    pub const BROWSER_FORWARD: u8 = 0xA7;
    pub const BROWSER_REFRESH: u8 = 0xA8;
    pub const BROWSER_STOP: u8 = 0xA9;
    pub const BROWSER_SEARCH: u8 = 0xAA;
    pub const BROWSER_FAVORITES: u8 = 0xAB;
    pub const BROWSER_HOME: u8 = 0xAC;
    pub const VOLUME_MUTE: u8 = 0xAD;
    pub const VOLUME_DOWN: u8 = 0xAE;
    pub const VOLUME_UP: u8 = 0xAF;
    pub const MEDIA_NEXT_TRACK: u8 = 0xB0;
    pub const MEDIA_PREV_TRACJ: u8 = 0xB1;
    pub const MEDIA_STOP: u8 = 0xB2;
    pub const MEDIA_PLAY_PAUSE: u8 = 0xB3;
    pub const LAUNCH_MAIL: u8 = 0xB4;
    pub const LAUNCH_MEDIA_SELECT: u8 = 0xB5;
    pub const LAUNCH_APP1: u8 = 0xB6;
    pub const LAUNCH_APP2: u8 = 0xB7;
    pub const OEM_1: u8 = 0xBA;
    pub const OEM_PLUS: u8 = 0xBB;
    pub const OEM_COMMA: u8 = 0xBC;
    pub const OEM_MINUS: u8 = 0xBD;
    pub const OEM_PERIOD: u8 = 0xBE;
    pub const OEM_2: u8 = 0xBF;
    pub const OEM_3: u8 = 0xC0;
    pub const OEM_4: u8 = 0xDB;
    pub const OEM_5: u8 = 0xDC;
    pub const OEM_6: u8 = 0xDD;
    pub const OEM_7: u8 = 0xDE;
    pub const OEM_8: u8 = 0xDF;
    pub const OEM_102: u8 = 0xE2;
    pub const PROCESS: u8 = 0xE5;
    pub const PACKET: u8 = 0xE7;
    pub const ATTN: u8 = 0xF6;
    pub const CRSEL: u8 = 0xF7;
    pub const EXSEL: u8 = 0xF8;
    pub const EREOF: u8 = 0xF9;
    pub const PLAY: u8 = 0xFA;
    pub const ZOOM: u8 = 0xFB;
    pub const PA1: u8 = 0xFD;
    pub const OEM_CLEAR: u8 = 0xFE;

    #[inline]
    pub fn new() -> Self {
        KeyBoard {
            paste_shortcut_key: [Self::LCTRL, Self::V, Self::EMPTY],
        }
    }

    ///send_text function need shortcut key of paste , default is left control + V
    #[inline]
    pub fn change_paste_shortcut_key(&mut self, keys: [u8; 3]) -> &Self {
        self.paste_shortcut_key = keys;
        self
    }
    ///press key
    pub fn press(&self, key: u8) -> &Self {
        if (key > u8::MAX) || (key < u8::MIN) {
            panic!("key code isn't exist!")
        }
        unsafe { Key_press(key as i32) };
        self
    }
    ///release key
    pub fn release(&self, key: u8) -> &Self {
        if (key > u8::MAX) || (key < u8::MIN) {
            panic!("key code isn't exist!")
        }
        unsafe { Key_release(key as i32) };
        self
    }
    ///send text to any box which can be input
    pub fn send_text(&self, text: &str) -> &Self {
        let mut a: Vec<u16> = text.encode_utf16().collect();
        a.push(0);

        unsafe {
            Key_text(
                a.as_ptr() as *const wchar_t,
                self.paste_shortcut_key.as_ptr(),
            )
        };
        self
    }
    ///wait in milliseconds
    #[inline]
    pub fn wait(&self, ms: u64) -> &Self {
        sleep(Duration::from_millis(ms));
        self
    }
}
