use bit_array::BitArray;
use rand::{rngs::StdRng, Rng};
use typenum::U8;
use crate::{backend::{as_nibbles, as_u4, bcd, Cpu, Mem}, terminal::Monochrome};

use super::{ins::Ins, mmu::MMU, Addr, SysIO, DEFAULT_PC};

#[derive(Debug)]
pub(crate) struct CPU {
    delay: u8, 
    index: Addr,
    pc: Addr,
    rd: StdRng,
    sound: u8,
    stack: Vec<Addr>,
    v: [u8; 16],
}

impl CPU {
    pub(crate) fn new(rd: StdRng) -> Self {
        CPU {
            delay: 0,
            index: 0,
            pc: DEFAULT_PC as Addr,
            rd,
            sound: 0,
            stack: Vec::new(),
            v: [0u8; 16],
        }
    }

    fn step(&mut self) {
        self.pc += self.ins_size() as u16;
    }

    fn step_back(&mut self) {
        self.pc -= self.ins_size() as u16;
    }

    fn step_if(&mut self, cond: bool) {
        if cond {
            self.step();
        }
    }
}

impl Cpu<Ins, MMU, SysIO, 2> for CPU {
    fn fetch(&mut self, mmu: &MMU) -> [u8; 2] {
        let b0 = mmu.read(self.pc);
        let b1 = mmu.read(self.pc + 1);
        self.step();
        [b0, b1]
    }
    
    fn decode(&self, buf: &[u8; 2]) -> Ins {
        let nn = buf[1];
        let nnn = (((buf[0] & 0x0F) as u16) << 8) + buf[1] as u16;
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
            [0x8, x, _, 0x6] => Ins::Op8XY6(x),
            [0x8, x, y, 0x7] => Ins::Op8XY7(x, y),
            [0x8, x, _, 0xE] => Ins::Op8XYE(x),
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
    
    fn execute(&mut self, mmu: &mut MMU, sysio: &mut SysIO, ins: Ins) {
        macro_rules! v {
            ($x:expr) => { self.v[$x as usize] };
        }

        macro_rules! i {
            ($x:expr) => { self.index + $x as u16 };
        }

        println!("{:?}", ins);

        match ins {
            Ins::Op00E0 => sysio.0.clear(),
            Ins::Op00EE => {
                if let Some(addr) = self.stack.pop() {
                    self.pc = addr;
                }
            },
            Ins::Op0NNN(_addr) => unimplemented!(),
            Ins::Op1NNN(addr) => self.pc = addr,
            Ins::Op2NNN(addr) => {
                self.stack.push(self.pc);
                self.pc = addr;
            },
            Ins::Op3XNN(x, nn) => self.step_if(v!(x) == nn),
            Ins::Op4XNN(x, nn) => self.step_if(v!(x) != nn),
            Ins::Op5XY0(x, y) => self.step_if(v!(x) == v!(y)),
            Ins::Op6XNN(x, nn) => v!(x) = nn,
            Ins::Op7XNN(x, nn) => v!(x) = v!(x).wrapping_add(nn),
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
            Ins::Op8XY6(x) => {
                let res = v!(x) & 0x01;
                v!(x) >>= 1;
                v!(0x0F) = res;
            }
            Ins::Op8XY7(x, y) => {
                let (vx, o) = v!(y).overflowing_sub(v!(x));
                v!(x) = vx;
                v!(0x0F) = !o as u8;
            },
            Ins::Op8XYE(x) => {
                let res = ((v!(x) & 0x80) != 0) as u8;
                v!(x) <<= 1;
                v!(0x0F) = res;
            },
            Ins::Op9XY0(x, y) => self.step_if(v!(x) != v!(y)),
            Ins::OpANNN(addr) => self.index = addr,
            Ins::OpBNNN(addr) => self.pc = addr + v!(0) as u16,
            Ins::OpCXNN(x, nn) => v!(x) = self.rd.random::<u8>() & nn,
            Ins::OpDXYN(x, y, n) => {
                v!(0xF) = 0;
                let x_pos = v!(x) % 64;
                let y_pos = v!(y) % 32;
                for row in 0..n {
                    let a = mmu.read(i!(row));
                    let bv = BitArray::<u8, U8>::from_bytes(&[a]);
                    for (col, bit) in bv.iter().enumerate() {
                        if bit {
                            let i = (row + y_pos) as usize;
                            let j = col + x_pos as usize;
                            if let Some(pixel) = sysio.0.0.get_mut(i, j) {
                                match pixel {
                                    Monochrome::White => *pixel = Monochrome::Black,
                                    Monochrome::Black => {
                                        *pixel = Monochrome::White;
                                        v!(0xF) = 1;
                                    },
                                }
                            }
                        }
                    }
                }
            },
            Ins::OpEX9E(x) => self.step_if(sysio.1.is_pressed(as_u4(v!(x)))),
            Ins::OpEXA1(x) => self.step_if(!sysio.1.is_pressed(as_u4(v!(x)))),
            Ins::OpFX07(x) => v!(x) = self.delay,
            Ins::OpFX0A(x) => match sysio.1.get_pressed() {
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
}