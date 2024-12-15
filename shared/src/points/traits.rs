pub trait Modulo<Rhs = Self> {
    type Output;

    fn modulo(self, rhs: Rhs) -> Self::Output;
}

pub trait ModuloAssign<Rhs = Self> {
    fn modulo_assign(&mut self, rhs: Rhs);
}

pub trait Absolute {
    fn absolute(self) -> Self;
}

pub trait ModuloPositive<Rhs = Self> {
    type Output;

    fn modulo_positive(self, rhs: Rhs) -> Self::Output;
}

pub trait ModuloPositiveAssign<Rhs = Self> {
    fn module_positive_assign(&mut self, rhs: Rhs);
}
