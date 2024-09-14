
/// Syntactic sugar to group constants of a new type.
macro_rules! newtype_values {
    (pub const $tp: ident { $($name: ident = $val: expr),* };) => {
        $ (
            pub const $name: $tp = $tp($val);
        )*
    };
}


/// Allows Range for primitive-like objects such as `Square`, `Rank` and `File`.
macro_rules! impl_step {
    ($tp:ty) => {
        impl std::iter::Step for $tp {
            fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                if start.0 > end.0 {
                    None
                } else {
                    Some((end.0 - start.0) as usize)
                }
            }
    
            fn forward_checked(start: Self, count: usize) -> Option<Self> {
                if (start.0 as usize) + count > 255 {
                    None
                } else {
                    Some(Self(start.0 + count as u8))
                }
            }
    
            fn backward_checked(start: Self, count: usize) -> Option<Self> {
                if (start.0 as isize) - (count as isize) < 0 {
                    None
                } else {
                    Some(Self(start.0 - count as u8))
                }
            }
        }
    };
}


/// This allows to convert from and to a char, as well as display,
/// for an enum-like type that may be represented as a char.
macro_rules! char_enum_conversions {
    (match $tp: ty { $($name: ident => $chr: literal),+ }) => {
        impl $tp {
            pub fn to_char(self) -> char {
                match self {
                    $(
                        <$tp>::$name => $chr,
                    )+
                }
            }
        }

        impl fmt::Display for $tp {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_char())
            }
        }

        impl TryFrom<char> for $tp {
            type Error = String;

            fn try_from(c: char) -> Result<$tp, Self::Error> {
                match c {
                    $(
                        $chr => Ok(<$tp>::$name),
                    )+
                    _ => Err(format!("Couldn't parse: `{}`", c))
                }
            }
        }
    };
}