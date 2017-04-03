#![feature(box_syntax)]

extern crate time;
extern crate image;

#[macro_use] mod macros;
mod gaia_entry;
mod timer;
mod generate_image;

pub use gaia_entry::{AstrometricPrior, GaiaEntry};
pub use timer::Timer;
pub use generate_image::generate_image;
