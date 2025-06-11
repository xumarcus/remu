mod brush;
mod monochrome;

use std::fmt::{Display, Write};
use array2d::Array2D;
use brush::Brush;
pub use monochrome::Monochrome;

pub trait Colored : Copy + Eq + Sized {
    type B: Brushable;
    type F: Brushable;
    
    fn background(self) -> Self::B;
    fn foreground(self) -> Self::F;
}

pub trait Brushable : Copy + Default + Eq {
    fn code(self) -> &'static str;
}

const LOWER_HALF_BLOCK: char = '\u{2584}';

pub struct Painted<'a, C: Colored>(pub &'a Array2D<C>);

impl<'a, C: Colored> Display for Painted<'a, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut brush_b = Brush::<C::B>::new();
        let mut brush_f = Brush::<C::F>::new();
        for i in (0..self.0.num_rows()).step_by(2) {
            for j in 0..self.0.num_columns() {
                brush_b.update(self.0[(i, j)].background(), f)?;
                brush_f.update(self.0[(i + 1, j)].foreground(), f)?;
                f.write_char(LOWER_HALF_BLOCK)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
