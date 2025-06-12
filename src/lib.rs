use std::{fs::File, io};

use array2d::Array2D;
use rand::{rngs::StdRng, SeedableRng};

mod backend;
// mod frontend;
mod terminal;

pub trait System : Sized {
    type C: Copy + Eq + Sized;

    fn from_rom_with_rd(f: &mut File, rd: StdRng) -> io::Result<Self>;
    fn from_rom(f: &mut File) -> io::Result<Self> {
        Self::from_rom_with_rd(f, StdRng::from_os_rng())
    }
    fn render(&self) -> &Array2D<Self::C>;
    fn tick(&mut self, event: Option<winit::event::DeviceEvent>);
}

pub use backend::schip::Schip;
pub use terminal::write_framebuffer;
