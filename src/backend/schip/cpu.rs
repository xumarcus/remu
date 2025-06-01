use rand::Rng;
use crate::{backend::{util::as_nibbles, Cpu, Mem, U4}, framebuffer, util::bcd, Framebuffer};

use super::{ins::Ins, mmu::MMU, Addr, Key};

struct CPU<R: Rng> {
    delay: u8,
    framebuffer: Framebuffer,
    index: Addr,
    pressed: Option<Key>,
    pc: Addr,
    rd: R,
    released: Option<Key>,
    sound: u8,
    v: [u8; 16],
}

impl<R: Rng> CPU<R> {
    const DEFAULT_PC: u16 = 0x200;

    pub(crate) fn new(rd: R) -> Self {
        CPU {
            delay: 0,
            framebuffer: Framebuffer::new(64, 32),
            index: 0,
            pressed: None,
            pc: Self::DEFAULT_PC,
            released: None,
            rd,
            sound: 0,
            v: [0u8; 16],
        }
    }

    fn step(&mut self) {
        self.pc += Self::FETCH_SIZE as u16;
    }

    fn step_back(&mut self) {
        self.pc -= Self::FETCH_SIZE as u16;
    }

    fn step_if(&mut self, cond: bool) {
        if cond {
            self.step();
        }
    }
}

impl<R: Rng> Cpu<Ins, MMU, 2> for CPU<R> {
    fn execute(&mut self, mmu: &mut MMU, ins: Ins) {
        macro_rules! v {
            ($x:expr) => { self.v[$x as usize] };
        }

        macro_rules! i {
            ($x:expr) => { self.index + $x as u16 };
        }

        match ins {
            Ins::Op0NNN(addr) => todo!(),
            Ins::Op00E0 => self.framebuffer = Framebuffer::new(64, 32),
            Ins::Op00EE => todo!(),
            Ins::Op1NNN(addr) => self.pc = addr,
            Ins::Op2NNN(addr) => todo!(),
            Ins::Op3XNN(x, nn) => self.step_if(v!(x) == nn),
            Ins::Op4XNN(x, nn) => self.step_if(v!(x) != nn),
            Ins::Op5XY0(x, y) => self.step_if(v!(x) == v!(y)),
            Ins::Op6XNN(x, nn) => v!(x) = nn,
            Ins::Op7XNN(x, nn) => v!(x) += nn,
            Ins::Op8XY0(x, y) => v!(x) = v!(y),
            Ins::Op8XY1(x, y) => v!(x) |= v!(y),
            Ins::Op8XY2(x, y) => v!(x) &= v!(y),
            Ins::Op8XY3(x, y) => v!(x) ^= v!(y),
            Ins::Op8XY4(x, y) => {
                let (vx, o) = v!(x).overflowing_add(v!(y));
                v!(x) = vx;
                v!(0x0F) = o as u8;
            },
            Ins::Op8XY5(x, y) => {
                let (vx, o) = v!(x).overflowing_sub(v!(y));
                v!(x) = vx;
                v!(0x0F) = !o as u8;
            },
            Ins::Op8XY6(x, _) => {
                v!(0x0F) = v!(x) & 0x01;
                v!(x) >>= 1;
            }
            Ins::Op8XY7(x, y) => {
                let (vx, o) = v!(y).overflowing_sub(v!(x));
                v!(x) = vx;
                v!(0x0F) = !o as u8;
            },
            Ins::Op8XYE(x, y) => {
                v!(0x0F) = ((v!(x) & 0x80) != 0) as u8;
                v!(x) <<= 1;
            },
            Ins::Op9XY0(x, y) => self.step_if(v!(x) != v!(y)),
            Ins::OpANNN(addr) => self.index = addr,
            Ins::OpBNNN(addr) => self.pc = addr + v!(0) as u16,
            Ins::OpCXNN(x, nn) => v!(x) = self.rd.random::<u8>() & nn,
            Ins::OpDXYN(x, y, n) => {
                v!(0xF) = 0;
                for i in 0..n {
                    let byte = mmu.read(i!(i));
                    for j in x..x+8 {
                        if (byte >> (7 - j) & 0x01) != 0 {
                            self.framebuffer[(i, j)].invert();
                            v!(0xF) = 1;
                        }
                    }
                }
            },
            Ins::OpEX9E(x) => self.step_if(self.pressed == Some(v!(x) & 0x0F as U4)),
            Ins::OpEXA1(x) => self.step_if(self.pressed != Some(v!(x) & 0x0F as U4)),
            Ins::OpFX07(x) => v!(x) = self.delay,
            Ins::OpFX0A(x) => match self.released {
                Some(key) => v!(x) = key,
                None => self.step_back(),
            },
            Ins::OpFX15(x) => self.delay = v!(x),
            Ins::OpFX18(x) => self.sound = v!(x),
            Ins::OpFX1E(x) => self.index += v!(x) as u16,
            Ins::OpFX29(x) => self.index = 5 * v!(x) as u16,
            Ins::OpFX33(x) => {
                let s = bcd(v!(x));
                for i in 0..3 {
                    mmu.write(i!(i), s[i]);
                }
            },
            Ins::OpFX55(x) => {
                for i in 0..=x {
                    let addr = self.index + i as u16;
                    mmu.write(i!(i), v!(i));
                }
            },
            Ins::OpFX65(x) => {
                for i in 0..=x {
                    v!(i) = mmu.read(i!(i));
                }
            }
        }
    }
    
    fn fetch(&mut self, mmu: &MMU) -> [u8; 2] {
        let b0 = mmu.read(self.pc);
        let b1 = mmu.read(self.pc + 1);
        self.step();
        [b0, b1]
    }
    
    fn decode(&self, buf: &[u8; 2]) -> Ins {
        let nn = buf[1];
        let nnn = u16::from_be_bytes(*buf);
        match as_nibbles(buf) {
            [0x0, 0x0, 0xE, 0x0] => Ins::Op00E0,
            [0x0, 0x0, 0xE, 0xE] => Ins::Op00EE,
            [0x0, _, _, _] => Ins::Op0NNN(nnn),
            [0x1, _, _, _] => Ins::Op1NNN(nnn),
            [0x2, _, _, _] => Ins::Op2NNN(nnn),
            [0x3, x, _, _] => Ins::Op3XNN(x, nn),
            [0x4, x, _, _] => Ins::Op4XNN(x, nn),
            [0x5, x, y, 0] => Ins::Op5XY0(x, y),
            [0x6, x, _, _] => Ins::Op6XNN(x, nn),
            [0x7, x, _, _] => Ins::Op7XNN(x, nn),
            [0x8, x, y, 0x0] => Ins::Op8XY0(x, y),
            [0x8, x, y, 0x1] => Ins::Op8XY1(x, y),
            [0x8, x, y, 0x2] => Ins::Op8XY2(x, y),
            [0x8, x, y, 0x3] => Ins::Op8XY3(x, y),
            [0x8, x, y, 0x4] => Ins::Op8XY4(x, y),
            [0x8, x, y, 0x5] => Ins::Op8XY5(x, y),
            [0x8, x, y, 0x6] => Ins::Op8XY6(x, y),
            [0x8, x, y, 0x7] => Ins::Op8XY7(x, y),
            [0x8, x, y, 0xE] => Ins::Op8XYE(x, y),
            [0x9, x, y, 0x0] => Ins::Op9XY0(x, y),
            [0xA, _, _, _] => Ins::OpANNN(nnn),
            [0xB, _, _, _] => Ins::OpBNNN(nnn),
            [0xC, x, _, _] => Ins::OpCXNN(x, nn),
            [0xD, x, y, n] => Ins::OpDXYN(x, y, n),
            [0xE, x, 0x9, 0xE] => Ins::OpEX9E(x),
            [0xE, x, 0xA, 0x1] => Ins::OpEXA1(x),
            [0xF, x, 0x0, 0x7] => Ins::OpFX07(x),    // LD Vx, DT
            [0xF, x, 0x0, 0xA] => Ins::OpFX0A(x),    // LD Vx, K
            [0xF, x, 0x1, 0x5] => Ins::OpFX15(x),    // LD DT, Vx
            [0xF, x, 0x1, 0x8] => Ins::OpFX18(x),    // LD ST, Vx
            [0xF, x, 0x1, 0xE] => Ins::OpFX1E(x),    // ADD I, Vx
            [0xF, x, 0x2, 0x9] => Ins::OpFX29(x),    // LD F, Vx
            [0xF, x, 0x3, 0x3] => Ins::OpFX33(x),    // LD B, Vx
            [0xF, x, 0x5, 0x5] => Ins::OpFX55(x),    // LD [I], Vx
            [0xF, x, 0x6, 0x5] => Ins::OpFX65(x),    // LD Vx, [I]
            _ => panic!()
        }
    }
}