mod util;
pub(crate) mod schip;

trait Cpu<I, M, const N: usize> {
    const FETCH_SIZE: usize = N;

    fn fetch(&mut self, mmu: &M) -> [u8; N];
    fn decode(&self, v: &[u8; N]) -> I;
    fn execute(&mut self, mmu: &mut M, ins: I);
}

trait Mem<T, V> {
    fn read(&self, addr: T) -> V;
    fn write(&mut self, addr: T, value: V);
}

type U4 = u8;
