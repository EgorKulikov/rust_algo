pub trait Invertible {
    type Output;

    fn inv(&self) -> Option<Self::Output>;
}
