use std::{fs::File, panic::AssertUnwindSafe, path::Path};

use remu::{Painted, Renderer, Schip, System};

fn main() -> std::io::Result<()> {
    let path = Path::new("test-roms/schip/IBM Logo.ch8");
    let mut f = File::open(path)?;
    let mut schip = Schip::from_rom(&mut f)?;
    
    /* Run the emulator until it panics
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
        loop {
            schip.tick();
        }
    }));
    println!("{:#?}", res);
    */ 

    for _ in 0..200 {
        schip.tick();
    }
    println!("{}", Painted(schip.framebuffer()));

    Ok(())
}
