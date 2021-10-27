#![no_std]
#![allow(unused_macros)]

macro_rules! option {
    () => {
        None
    };
    ($x:expr) => {
        Some($x)
    };
}

#[macro_export]
macro_rules! const_enum_expr {
    ($(#[$attr:meta])* $v:vis $name:ident<$t:ty> { $($variant:ident($x:expr) $(=$d:literal)?),+ $(,)? }) => {
        $(#[$attr])*
        $v enum $name {
            $($variant$(=$d)?),+
        }
        #[allow(dead_code)]
        impl $name {
            pub const fn value(&self) -> $t {
                match self {
                    $(Self::$variant => $x),+
                }
            }
        }
    };
    // matcher for enums with optional expressions
    ($(#[$attr:meta])* $v:vis $name:ident<$t:ty> { $($variant:ident$(($x:expr))? $(=$d:literal)?),+ $(,)? }) => {
        $(#[$attr])*
        $v enum $name {
            $($variant$(=$d)?),+
        }
        #[allow(dead_code)]
        impl $name {
            pub const fn value(&self) -> Option<$t> {
                match self {
                    $(Self::$variant => option!($($x)?)),+
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    const_enum_expr! {
        #[allow(dead_code)]
        pub NoTrailingCommaCompiles<()> {
            A(()),
            B(())
        }
    }

    const_enum_expr! {
        #[allow(dead_code)]
        pub OptionalExpr<usize> {
            A,
            B(3),
        }
    }

    const_enum_expr! {
        #[allow(dead_code)]
        pub Pointless<usize> {
            A,
            B
        }
    }

    const_enum_expr! {
        #[allow(dead_code)]
        pub EvenMorePointless<usize> {
            A
        }
    }

    const_enum_expr! {
        #[allow(dead_code)]
        pub Single<&str> {
            A("hi")
        }
    }

    const_enum_expr! {
        #[allow(dead_code)]
        pub SingleTrailing<&str> {
            A("ho"),
        }
    }

    #[test]
    fn const_enum_expr_determinant_assignment() {
        const_enum_expr! {
            WithDeterminant<&str> {
                One("Foo"),
                Two("Bar") = 42
            }
        }

        assert_eq!(WithDeterminant::Two as u32, 42);
        assert_eq!(WithDeterminant::One.value(), "Foo");
    }

    #[test]
    fn const_enum_expr_constness() {
        const_enum_expr! {
            SpecialNumbers<u32> {
                Answer(42),
                Leet(1337),
                Devil(666)
            }
        }

        const_enum_expr! {
            Stuff<u32> {
                A(SpecialNumbers::Answer.value()),
                B(SpecialNumbers::Devil as u32),
            }
        }

        const LEET: u32 = SpecialNumbers::Leet.value();
        assert_eq!(Stuff::A.value(), 42);
        assert_eq!(Stuff::B.value(), 2);
        assert_eq!(LEET, 1337);
    }

    #[test]
    fn const_enum_expr_using_another_one() {
        const_enum_expr! {
            #[derive(PartialEq, Eq, Debug)]
            SpecialNumbers<u32> {
                Answer(42),
                Leet(1337),
            }
        }

        const_enum_expr! {
            MaybeAnswer<SpecialNumbers> {
                A(SpecialNumbers::Answer),
                B,
            }
        }

        assert_eq!(MaybeAnswer::A.value(), Some(SpecialNumbers::Answer));
        assert_eq!(
            MaybeAnswer::A.value().unwrap().value(),
            SpecialNumbers::Answer.value()
        );
        assert_eq!(MaybeAnswer::B.value(), None);
        assert_eq!(SpecialNumbers::Leet.value(), 1000 + 337);
    }

    #[test]
    fn const_enum_expr_with_slice() {
        const_enum_expr! {
            #[allow(dead_code)]
            Resource<&[u8]> {
                FavIcon(&[1, 2, 3, 4]),
                Banner(&[123]),
            }
        }

        assert_eq!(Resource::FavIcon.value().len(), 4);
    }
}
