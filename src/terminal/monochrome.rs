use crate::terminal::Colored;

use super::Brushable;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Monochrome {
    White,
    Black
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct B(Monochrome);

impl Default for B {
    fn default() -> Self {
        B(Monochrome::Black)
    }
}

impl Brushable for B {
    fn code(self) -> &'static str {
        match self.0 {
            Monochrome::White => "\x1b[47m",
            Monochrome::Black => "\x1b[40m"
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct F(Monochrome);

impl Default for F {
    fn default() -> Self {
        F(Monochrome::White)
    }
}

impl Brushable for F {
    fn code(self) -> &'static str {
        match self.0 {
            Monochrome::White => "\x1b[97m",
            Monochrome::Black => "\x1b[30m"
        }
    }
}

impl Colored for Monochrome {
    type B = B;
    type F = F;
    
    fn background(self) -> Self::B {
        B(self)
    }

    fn foreground(self) -> Self::F {
        F(self)
    }
}
