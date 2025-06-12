use std::io;

use cpu::CPU;
use framebuffer::Framebuffer;
use keypad::Keypad;
use mmu::MMU;
use rand::rngs::StdRng;
use winit::event::DeviceEvent;

use crate::{terminal::Monochrome, System};

use super::{Cpu, u4};

mod cpu;
mod framebuffer;
mod ins;
mod keypad;
mod mmu;

type Addr = u16;
type Key = u4;
type Reg = u4;

const DEFAULT_PC: usize = 0x200;

pub(crate) struct SysIO(Framebuffer, Keypad);
pub struct Schip {
    cpu: CPU,
    mmu: MMU,
    sysio: SysIO,
}

impl System for Schip {
    type C = Monochrome;

    fn tick(&mut self, event: Option<DeviceEvent>) {
        if let Some(DeviceEvent::Key(e)) = event {
            self.sysio.1.handle_key(e);
        }
        let opcode = self.cpu.fetch(&self.mmu);
        let ins = self.cpu.decode(&opcode);
        self.cpu.execute(&mut self.mmu, &mut self.sysio, ins);
    }

    fn from_rom_with_rd(f: &mut std::fs::File, rd: StdRng) -> io::Result<Self> {
        Ok(Self {
            cpu: CPU::new(rd),
            mmu: MMU::new(f)?,
            sysio: SysIO(Framebuffer::default(), Keypad::default())
        })
    }
    
    fn render(&self) -> &array2d::Array2D<Self::C> {
        &self.sysio.0.0
    }
}

