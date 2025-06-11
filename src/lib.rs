use std::{fs::File, io};

use array2d::Array2D;
use rand::{rngs::StdRng, SeedableRng};

mod backend;
// mod frontend;
mod terminal;
mod util;

pub trait Renderer {
    type C: Copy + Eq + Sized;

    fn framebuffer(&self) -> &Array2D<Self::C>;
}

pub trait System : Renderer + Sized {
    fn device_event(&mut self, event: winit::event::DeviceEvent);
    fn tick(&mut self);
    fn from_rom(f: &mut File) -> io::Result<Self> {
        Self::from_rom_with_rd(f, StdRng::from_os_rng())
    }
    fn from_rom_with_rd(f: &mut File, rd: StdRng) -> io::Result<Self>;
}

pub use backend::schip::Schip;
pub use terminal::Painted;
