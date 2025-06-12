use termcolor::Color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Monochrome {
    White,
    Black
}

impl Into<Color> for Monochrome {
    fn into(self) -> Color {
        match self {
            Monochrome::White => Color::White,
            Monochrome::Black => Color::Black,
        }
    }
}
