pub(crate) const fn bcd(x: u8) -> [u8; 3] {
    [x / 100, (x / 10) % 10, x % 10]
}
