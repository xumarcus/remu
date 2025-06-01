use std::fs::File;

use framebuffer::Framebuffer;

use winit::event::DeviceEvent;

mod backend;
mod util;
mod framebuffer;
mod frontend;

trait Clock {
    fn tick(&mut self);
}

trait System : Clock + Sized {
    fn with_rom(f: File) -> Option<Self>;
    fn frame_buffer(&self) -> &Framebuffer;
    fn device_event(&mut self, event: &DeviceEvent);
}
