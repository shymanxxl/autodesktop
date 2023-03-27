#![allow(unused)]
use libc::c_int;

pub struct Screen;
#[link(name = "low")]
extern "C" {
    fn Screen_get_width() -> c_int;
    fn Screen_get_height() -> c_int;

}

impl Screen {
    #[inline]
    pub fn width() -> i32 {
        unsafe { Screen_get_width() }
    }

    #[inline]
    pub fn height() -> i32 {
        unsafe { Screen_get_height() }
    }

    #[inline]
    pub fn rect() -> (i32, i32) {
        (unsafe { Screen_get_width() }, unsafe {
            Screen_get_height()
        })
    }
}
