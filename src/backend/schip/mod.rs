use std::io;

use array2d::Array2D;
use cpu::CPU;
use mmu::MMU;
use rand::rngs::StdRng;
use winit::{event::{DeviceEvent, RawKeyEvent}, keyboard::{KeyCode, PhysicalKey}};

use crate::{terminal::Monochrome, Renderer, System};

use super::{Cpu, u4};

mod cpu;
mod ins;
mod mmu;

type Addr = u16;
type Key = u4;
type Reg = u4;

const DEFAULT_PC: usize = 0x200;

pub struct Schip {
    cpu: CPU,
    mmu: MMU,
}

impl Renderer for Schip {
    type C = Monochrome;

    fn framebuffer(&self) -> &Array2D<Self::C> {
        self.cpu.framebuffer()
    }
}

impl System for Schip {
    fn device_event(&mut self, event: DeviceEvent) {
        use winit::event::ElementState;
        match event {
            DeviceEvent::Key(RawKeyEvent { physical_key: pk, state }) => match state {
                ElementState::Pressed => self.cpu.update_pressed(from_pk(pk)),
                ElementState::Released => self.cpu.update_released(from_pk(pk)),
            },
            _ => ()
        }
    }

    fn tick(&mut self) {
        let opcode = self.cpu.fetch(&self.mmu);
        let ins = self.cpu.decode(&opcode);
        self.cpu.execute(&mut self.mmu, ins);
    }

    fn from_rom_with_rd(f: &mut std::fs::File, rd: StdRng) -> io::Result<Self> {
        Ok(Self {
            cpu: CPU::new(rd),
            mmu: MMU::new(f)?
        })
    }
}

const fn from_pk(pk: PhysicalKey) -> Option<Key> {
    /*
     * 1->1 2->2 3->3 4->C
     * Q->4 W->5 E->6 R->D
     * A->7 S->8 D->9 F->E
     * Z->A X->0 C->B V->F
     */
    match pk {
        PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
        PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
        PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
        PhysicalKey::Code(KeyCode::Digit4) => Some(0xC),

        PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
        PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
        PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
        PhysicalKey::Code(KeyCode::KeyR) => Some(0xD),

        PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
        PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
        PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
        PhysicalKey::Code(KeyCode::KeyF) => Some(0xE),

        PhysicalKey::Code(KeyCode::KeyZ) => Some(0xA),
        PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
        PhysicalKey::Code(KeyCode::KeyC) => Some(0xB),
        PhysicalKey::Code(KeyCode::KeyV) => Some(0xF),

        _ => None
    }
}