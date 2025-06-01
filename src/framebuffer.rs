use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub(crate) struct RGB(pub [u8; 3]);

impl RGB {
    pub(crate) fn invert(&mut self) {
        for i in 0..3 {
            self.0[i] = 0xFF - self.0[i];
        }
    }
}

pub(crate) struct Framebuffer {
    data: Vec<RGB>,
    height: usize,
    width: usize,
}

impl Framebuffer {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![RGB([0u8; 3]); width * height],
            width,
            height
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.width * self.height
    }
}

impl<T1: Into<usize>, T2: Into<usize>> Index<(T1, T2)> for Framebuffer {
    type Output = RGB;

    fn index(&self, index: (T1, T2)) -> &Self::Output {
        &self.data[index.0.into() + index.1.into() * self.width]
    }
}

impl<T1: Into<usize>, T2: Into<usize>> IndexMut<(T1, T2)> for Framebuffer {
    fn index_mut(&mut self, index: (T1, T2)) -> &mut Self::Output {
        &mut self.data[index.0.into() + index.1.into() * self.width]
    }
}
