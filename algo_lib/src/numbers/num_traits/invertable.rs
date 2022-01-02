pub trait Invertable: Sized {
    type Output;

    fn inv(&self) -> Option<Self>;
}
