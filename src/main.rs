use std::{fs::File, path::Path};

use remu::{write_framebuffer, Schip, System};
use termcolor::StandardStream;
use winit::{event::RawKeyEvent, keyboard::KeyCode};

fn main() -> std::io::Result<()> {
    let path = Path::new("test-roms/schip/5-quirks.ch8");
    let mut f = File::open(path)?;
    let mut schip = Schip::from_rom(&mut f)?;

    for _ in 0..2000 {
        schip.tick(None);
    }
    schip.tick(Some(winit::event::DeviceEvent::Key(RawKeyEvent {
        physical_key: winit::keyboard::PhysicalKey::Code(KeyCode::KeyA),
        state: winit::event::ElementState::Pressed
    })));
    for _ in 0..2000 {
        schip.tick(None);
    }
    
    let mut out = StandardStream::stdout(termcolor::ColorChoice::Always);
    write_framebuffer(&mut out, schip.render())?;

    Ok(())
}
