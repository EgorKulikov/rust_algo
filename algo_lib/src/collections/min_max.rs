pub trait MinimMaxim<Rhs = Self>: PartialOrd + Sized {
    fn minim(&mut self, other: Rhs) -> bool;

    fn maxim(&mut self, other: Rhs) -> bool;
}

impl<T: PartialOrd + Sized> MinimMaxim for T {
    fn minim(&mut self, other: Self) -> bool {
        if other < *self {
            *self = other;
            true
        } else {
            false
        }
    }

    fn maxim(&mut self, other: Self) -> bool {
        if other > *self {
            *self = other;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd + Sized> MinimMaxim<T> for Option<T> {
    fn minim(&mut self, other: T) -> bool {
        match self {
            None => {
                *self = Some(other);
                true
            }
            Some(v) => v.minim(other),
        }
    }

    fn maxim(&mut self, other: T) -> bool {
        match self {
            None => {
                *self = Some(other);
                true
            }
            Some(v) => v.maxim(other),
        }
    }
}
