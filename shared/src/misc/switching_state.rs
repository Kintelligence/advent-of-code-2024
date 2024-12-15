pub struct SwitchingState<T> {
    a: T,
    b: T,
    a_to_b: bool,
}

impl<T> SwitchingState<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b, a_to_b: true }
    }

    pub fn states(&mut self) -> (&mut T, &mut T) {
        return if self.a_to_b {
            (&mut self.a, &mut self.b)
        } else {
            (&mut self.b, &mut self.a)
        };
    }

    pub fn current(&mut self) -> &mut T {
        return if self.a_to_b {
            &mut self.a
        } else {
            &mut self.b
        };
    }

    pub fn next(&mut self) -> &mut T {
        return if self.a_to_b {
            &mut self.b
        } else {
            &mut self.a
        };
    }

    pub fn switch(&mut self) {
        self.a_to_b = !self.a_to_b;
    }
}
