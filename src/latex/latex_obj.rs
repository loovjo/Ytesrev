extern crate sdl2;

use sdl2::{render::Canvas, video::Window, pixels::Color};
use image::{PngImage, KnownSize, ImageContainer};
use drawable::{Drawable, Position, State, DrawSettings};
use super::render::{register_equation, read_image, LatexIdx};


pub struct LatexObj {
    pub inner: Option<PngImage>,
    pub id: Option<LatexIdx>,
    pub expr: &'static str,
    pub is_text: bool,
}

impl KnownSize for LatexObj {
    fn width(&self) -> usize {
        if let Some(ref inner) = self.inner {
            inner.width()
        } else {
            0
        }
    }
    fn height(&self) -> usize {
        if let Some(ref inner) = self.inner {
            inner.height()
        } else {
            0
        }
    }
}

impl ImageContainer for LatexObj {
    fn get_data(&self) -> &Vec<u8> {
        if let Some(ref inner) = self.inner {
            inner.get_data()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
    fn get_data_mut(&mut self) -> &mut Vec<u8> {
        if let Some(ref mut inner) = self.inner {
            inner.get_data_mut()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
    fn into_data(self) -> Vec<u8> {
        if let Some(inner) = self.inner {
            inner.into_data()
        } else {
            panic!("Use of imagecontainer on unloaded LatexObj");
        }
    }
}

impl LatexObj {
    pub fn math(expr: &'static str) -> LatexObj {
        LatexObj {
            inner: None,
            id: None,
            expr,
            is_text: false,
        }
    }

    pub fn text(expr: &'static str) -> LatexObj {
        LatexObj {
            inner: None,
            id: None,
            expr: expr,
            is_text: true,
        }
    }
}

impl Drawable for LatexObj {
    fn content(&self) -> Vec<&dyn Drawable> {
        if let Some(ref inner) = self.inner {
            vec![ inner ]
        } else {
            vec![ ]
        }
    }
    fn content_mut(&mut self) -> Vec<&mut dyn Drawable> {
        if let Some(ref mut inner) = self.inner {
            vec![ inner ]
        } else {
            vec![ ]
        }
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, position: &Position, settings: DrawSettings) {
        if let Some(ref mut img) = self.inner {
            img.draw(canvas, position, settings);
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 255));
            let rect = position.into_rect_with_size(100, 100);
            canvas.fill_rect(rect).expect("Can't draw");
        }
    }

    fn register(&mut self) {
        self.id = Some(register_equation(self.expr, self.is_text));
    }

    fn load(&mut self) {
        if let Some(id) = self.id.take() {
            match read_image(id) {
                Ok(image) => {
                    self.inner = Some(image);
                }
                Err(e) => {
                    eprintln!("Couldn't load image for expression `{}`: {:?}", self.expr, e);
                }
            }
        } else {
            eprintln!("Wrong loading order!");
        }
    }

    fn step(&mut self) {}
    fn state(&self) -> State { State::Final }
}
