use super::Brushable;

pub(crate) struct Brush<T: Brushable>(T);

impl<T: Brushable> Brush<T> {
    pub(crate) fn new() -> Self {
        Brush(T::default())
    }

    pub(crate) fn update(&mut self, t: T, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 != t {
            self.0 = t;
            f.write_str(t.code())?;
        }
        Ok(())
    }
}
