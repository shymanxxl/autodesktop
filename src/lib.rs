//! # Introduce
//! Under Windows system, a set of tools to simulate the operation of a mouse and keyboard



//! # Install
//! 1.need mingw toolchain and set it to PATH environment variable
//! 
//! 2.set rust toolchain
//! ```
//! rustup default stable-x86_64-pc-windows-gnu 
//! ```


//! # Tutorial

//! ## Mouse
//! ### Simple usage
//! ```
//! Mouse::new()
//!         .move_to(0,0)
//!         .unwrap()
//!         .click();
//! ```

//! ## KeyBoard
//! ### Simple usage
//! When you using vim ,the next code will change vim to the insert mode and input "大家好!"
//! ```
//! KeyBoard::new()
//!         .press(KeyBoard::I)
//!         .release(KeyBoard::I)
//!         .send_text("大家好!");
//! ```

mod error;
mod keyboard;
mod mouse;
mod screen;

pub use error::MouseError;
///simulate keyboard input
pub use keyboard::KeyBoard;
///simulate mouse input
pub use mouse::Mouse;
///get screen size
pub use screen::Screen;
