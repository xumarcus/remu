mod util;
pub(crate) mod schip;

trait Cpu<I, M, const INS_SIZE: usize> {
    fn ins_size(&self) -> usize {
        INS_SIZE
    } // TODO perhaps we should have step* methods here
    fn fetch(&mut self, mmu: &M) -> [u8; INS_SIZE];
    fn decode(&self, v: &[u8; INS_SIZE]) -> I;
    fn execute(&mut self, mmu: &mut M, ins: I);
}

trait Mem<T, V> {
    fn read(&self, addr: T) -> V;
    fn write(&mut self, addr: T, value: V);
}

#[allow(non_camel_case_types)]
type u4 = u8;
