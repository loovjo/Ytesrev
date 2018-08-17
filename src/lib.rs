//! # Ytesrev
//!
//! Ytesrev is a library to create presentations programmatically in rust. It is inspiered by
//! the tool [manim] by 3Blue1Brown, but can be used in live situations where dynamicity matters.
//!
//! ---
//!
//! ## Important Note:
//!
//! Ytesrev is *extremely* slow on debug build, please run it release mode by running
//! `cargo run --release`. On debug build, ytesrev can go down to as low as ~3 FPS and take over 5
//! minutes to load a simple presentation, while in release mode, it never drops below 60 FPS, and
//! takes under 15 seconds to load.
//!
//! [manim]: https://github.com/3b1b/manim


#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate png;
extern crate rand;
extern crate sdl2;

pub mod anchor;
pub mod ditherer;
pub mod drawable;
pub mod empty;
pub mod image;
pub mod latex;
pub mod layout;
pub mod margin;
pub mod scene;
pub mod solid;
pub mod window;
pub mod withsize;
pub use sdl2::rect::{Rect, Point};

pub mod prelude {
    //! A "prelude" to avoid having to `use` a gazillion different things
    pub use sdl2::rect::{Rect, Point};
    pub use anchor::{Anchor, AnchorDirection};
    pub use ditherer::{DitherDirection, Ditherer};
    pub use drawable::Drawable;
    pub use empty::Empty;
    pub use image::PngImage;
    pub use latex::LatexObj;
    pub use layout::layered::Layered;
    pub use layout::split::{SplitPrec, UpdateOrder};
    pub use layout::stack::{ElementPositioning, Stack};
    pub use layout::Orientation;
    pub use margin::Margin;
    pub use scene::{DrawableWrapper, Scene};
    pub use solid::Solid;
    pub use window::WindowManager;
    pub use withsize::WithSize;
}
