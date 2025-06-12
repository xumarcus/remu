use std::array;

pub(crate) mod schip;

trait Cpu<I, M, S, const INS_SIZE: usize> {
    fn ins_size(&self) -> usize {
        INS_SIZE
    } // TODO perhaps we should have step* methods here
    fn fetch(&mut self, mmu: &M) -> [u8; INS_SIZE];
    fn decode(&self, v: &[u8; INS_SIZE]) -> I;
    fn execute(&mut self, mmu: &mut M, sysio: &mut S, ins: I);
}

trait Mem<T, V> {
    fn read(&self, addr: T) -> V;
    fn write(&mut self, addr: T, value: V);
}

#[allow(non_camel_case_types)]
type u4 = u8;

pub(crate) fn as_u4<T: Into<u8>>(x: T) -> u4 {
    let z: u8 = x.into();
    z & 0x0F
}

pub(crate) fn as_nibbles<const N: usize>(buf: &[u8]) -> [u4; N] {
    array::from_fn(|i| {
        let byte = buf[i / 2];
        if i % 2 == 0 {
            (byte & 0xF0) >> 4
        } else {
            byte & 0x0F
        }
    })
}

pub(crate) const fn bcd(x: u8) -> [u8; 3] {
    [x / 100, (x / 10) % 10, x % 10]
}
