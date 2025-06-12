use array2d::Array2D;

use crate::terminal::Monochrome;

pub(crate) struct Framebuffer(pub Array2D<Monochrome>);

impl Default for Framebuffer {
    fn default() -> Self {
        Self(Array2D::filled_with(Monochrome::White, 32, 64))
    }
}

impl Framebuffer {
    pub(crate) fn clear(&mut self) {
        *self = Self::default();
    }
}
