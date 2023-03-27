#![allow(unused)]
use crate::error;
use crate::Screen;
use libc::{c_int, c_long, wchar_t};
use std::borrow::Borrow;
use std::fmt::Error;
use std::thread::sleep;
use std::time::Duration;
pub struct Mouse {
    pub x_range: (i32, i32),
    pub y_range: (i32, i32),
}
#[repr(C)]
struct RECTL {
    pub left: c_long,
    pub top: c_long,
    pub right: c_long,
    pub bottom: c_long,
}

#[link(name = "low")]
extern "C" {
    fn Mouse_get_pos_x() -> c_int;
    fn Mouse_get_pos_y() -> c_int;
    fn Mouse_move_to(x: c_int, y: c_int) -> c_int;
    fn Mouse_left_press();
    fn Mouse_left_release();
    fn Mouse_right_press();
    fn Mouse_right_release();
    fn Mouse_middle_press();
    fn Mouse_middle_release();
    fn Mouse_middle_scroll(s: c_int) -> c_int;
    fn Mouse_lock_window(window_name: *const wchar_t, rect: *const RECTL) -> c_int;

}

impl Mouse {
    #[inline]
    pub fn new() -> Self {
        Mouse {
            x_range: (0, Screen::width()),
            y_range: (0, Screen::height()),
        }
    }
    ///get mouse position
    #[inline]
    pub fn x() -> i32 {
        unsafe { Mouse_get_pos_x() }
    }
    ///get mouse position
    #[inline]
    pub fn y() -> i32 {
        unsafe { Mouse_get_pos_y() }
    }
    /// Move to an absolute point relative to the screen
    pub fn move_to(&self, x: i32, y: i32) -> error::Result<&Self> {
        if ((x < self.x_range.0) || (x > self.x_range.1))
            && ((y < self.y_range.0) || (x > self.y_range.1))
        {
            return Err(error::MouseError::LockXYOver(
                "x y both are out of the lock range!",
            ));
        }

        if (x < self.x_range.0) || (x > self.x_range.1) {
            return Err(error::MouseError::LockXOver(
                "x value are out of the lock range!",
            ));
        }
        if (y < self.y_range.0) || (y > self.y_range.1) {
            return Err(error::MouseError::LockYOver(
                "y value are out of the lock range!",
            ));
        }

        match unsafe { Mouse_move_to(x, y) } {
            0 => {
                panic!("x and y input values are out of the screen range!")
            }
            _ => {}
        }
        Ok(self)
    }
    ///press left button of mouse
    #[inline]
    pub fn left_press(&self) -> &Self {
        unsafe { Mouse_left_press() };
        self
    }
    ///release left button of mouse
    #[inline]
    pub fn left_release(&self) -> &Self {
        unsafe { Mouse_left_release() };
        self
    }
    ///press right button of mouse
    #[inline]
    pub fn right_press(&self) -> &Self {
        unsafe { Mouse_right_press() };
        self
    }
    ///release right button of mouse
    #[inline]
    pub fn right_release(&self) -> &Self {
        unsafe { Mouse_right_release() };
        self
    }
    ///press middle button of mouse
    #[inline]
    pub fn middle_press(&self) -> &Self {
        unsafe { Mouse_middle_press() };
        self
    }

    ///release middle button of mouse
    #[inline]
    pub fn middle_release(&self) -> &Self {
        unsafe { Mouse_middle_release() };
        self
    }

    ///scroll the mouse wheel
    #[inline]
    pub fn scroll(&self, s: i32) -> &Self {
        match unsafe { Mouse_middle_scroll(s) } {
            0 => {
                panic!("s over the maximum or minimum")
            }
            _ => {}
        }
        self
    }
    ///click by mouse of left
    #[inline]
    pub fn click(&self) -> &Self {
        self.left_press().left_release()
    }
    ///double click by mouse of left
    #[inline]
    pub fn double_click(&self) -> &Self {
        self.left_press().left_release().left_press().left_release()
    }
    ///click by mouse of right
    #[inline]
    pub fn right_click(&self) -> &Self {
        self.right_press().right_release()
    }
    ///move the mouse relative to its current position
    #[inline]
    pub fn move_rel(&self, dx: i32, dy: i32) -> error::Result<&Self> {
        self.move_to(Self::x() + dx, Self::y() + dy)?;

        Ok(self)
    }
    ///wait in milliseconds
    #[inline]
    pub fn wait(&self, ms: u64) -> &Self {
        sleep(Duration::from_millis(ms));
        self
    }
    ///lock a window by window name(title), and move the mouse to the center of the window
    pub fn lock(&mut self, window_name: &str) -> Option<&Self> {
        let mut window_name: Vec<u16> = window_name.encode_utf16().collect();
        window_name.push(0);

        let mut rect = RECTL {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
        };

        let r = unsafe {
            Mouse_lock_window(
                window_name.as_ptr() as *const wchar_t,
                &mut rect as *mut RECTL,
            )
        };

        match r {
            0 => None,
            _ => {
                self.x_range = (rect.left, rect.right);
                self.y_range = (rect.top, rect.bottom);
                self.move_to(
                    (self.x_range.0 + self.x_range.1) / 2,
                    (self.y_range.0 + self.y_range.1) / 2,
                )
                .unwrap();
                Some(self)
            }
        }
    }
    /// unlock window ,back to initial state
    #[inline]
    pub fn unlock(&mut self) -> Self {
        Self {
            x_range: (0, Screen::width()),
            y_range: (0, Screen::height()),
        }
    }
}
