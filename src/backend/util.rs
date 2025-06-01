use std::array;

use super::U4;

pub(crate) fn as_nibbles<const N: usize>(buf: &[u8]) -> [U4; N] {
    array::from_fn(|i| {
        let byte = buf[i / 2];
        if i % 2 == 0 {
            (byte & 0xF0) >> 4
        } else {
            byte & 0x0F
        }
    })
}