pub trait Modulo<Rhs = Self> {
    type Output;

    fn modulo(self, rhs: Rhs) -> Self::Output;
}
