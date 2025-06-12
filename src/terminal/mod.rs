mod monochrome;

use std::io::{self, Write};

use array2d::Array2D;
pub use monochrome::Monochrome;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

const LOWER_HALF_BLOCK: char = '\u{2584}';
fn write_stack<C: Copy + Into<Color>>(out: &mut StandardStream, upper: C, lower: C) -> io::Result<()> {
    out.set_color(ColorSpec::new()
        .set_bg(Some(upper.into()))
        .set_fg(Some(lower.into())))?;
    write!(out, "{}", LOWER_HALF_BLOCK)?;
    Ok(())
}

pub fn write_framebuffer<C: Copy + Into<Color>>(out: &mut StandardStream, buf: &Array2D<C>) -> io::Result<()> {
    for i in (0..buf.num_rows()).step_by(2) {
        for j in 0..buf.num_columns() {
            write_stack(out, buf[(i, j)], buf[(i + 1, j)])?;
        }
        out.reset()?;
        writeln!(out)?;
    }
    Ok(())
}
