pub trait Detuple {
    type Res;

    fn detuple(self) -> Self::Res;
}

macro_rules! detuple_impl {
    ($($t: ident $var: ident $val: ident),+) => {
        impl<$($t),+> Detuple for Vec<($($t),+)> {
            type Res = ($(Vec<$t>),+);

            fn detuple(self) -> Self::Res {
                $(let mut $var = Vec::with_capacity(self.len());)+
                for ($($val),+) in self {
                    $($var.push($val);)+
                }
                ($($var),+)
            }
        }
    };
}

detuple_impl!(A a aa, B b bb);
detuple_impl!(A a aa, B b bb, C c cc);
detuple_impl!(A a aa, B b bb, C c cc, D d dd);
