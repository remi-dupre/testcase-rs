use crate::Error;
use std::convert::TryInto;
use std::io::BufRead;
use std::str::FromStr;

// ---
// --- InlineTestCase
// ---

/// Trait for a testcase that can be parsed from a single line.
pub trait InlineTestCase: Sized {
    fn parse_line(buffer: &str) -> Result<Self, Error>;
}

impl InlineTestCase for String {
    fn parse_line(buffer: &str) -> Result<Self, Error> {
        Ok(buffer.trim().to_string())
    }
}

macro_rules! impl_inline_for_primitives {
    ( $( $type: ty ),+ ) => {
        $(
            impl InlineTestCase for $type {
                fn parse_line(buffer: &str) -> Result<Self, Error> {
                    buffer.parse().map_err(Into::into)
                }
            }
        )+
    };
}

impl_inline_for_primitives!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! impl_inline_for_tuples {
    ( $( ( $( $T: ident ),+ ) ),+ ) => {
        $(
            impl<$($T),+> InlineTestCase for ($($T,)+)
                where $(
                    $T: std::fmt::Debug + FromStr + 'static,
                    $T::Err: std::error::Error + 'static,
                )+
            {
                #[allow(non_snake_case)]
                fn parse_line(buffer: &str) -> Result<Self, Error> {
                    let mut line = buffer.split_whitespace();
                    $( let $T = line.next().ok_or(concat!("missing ", stringify!($T)))?.parse()?; )+
                    Ok(($($T,)+))
                }
            }
        )+
    };
}

impl_inline_for_tuples!(
    (T),
    (T, U),
    (T, U, V),
    (T, U, V, W),
    (T, U, V, W, X),
    (T, U, V, W, X, Y),
    (A, T, U, V, W, X, Y),
    (A, B, T, U, V, W, X, Y),
    (A, B, C, T, U, V, W, X, Y),
    (A, B, C, D, T, U, V, W, X, Y),
    (A, B, C, D, E, T, U, V, W, X, Y),
    (A, B, C, D, E, F, T, U, V, W, X, Y),
    (A, B, C, D, E, F, G, T, U, V, W, X, Y),
    (A, B, C, D, E, F, G, H, T, U, V, W, X, Y),
    (A, B, C, D, E, F, G, H, I, T, U, V, W, X, Y),
    (A, B, C, D, E, F, G, H, I, J, T, U, V, W, X, Y)
);

impl<T, const N: usize> InlineTestCase for [T; N]
where
    T: std::fmt::Debug + FromStr + 'static,
    T::Err: std::error::Error + 'static,
{
    fn parse_line(buffer: &str) -> Result<Self, Error> {
        buffer
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<T>, T::Err>>()?
            .try_into()
            .map_err(|vec| format!("array of invalid size: {:?}", vec).into())
    }
}

impl<T: FromStr> InlineTestCase for Vec<T>
where
    T::Err: std::error::Error + 'static,
{
    fn parse_line(buffer: &str) -> Result<Self, Error> {
        buffer
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()
            .map_err(Into::into)
    }
}

// ---
// --- BlockTestCase
// ---

/// Trait for a testcase that will consume a fixed number of lines.
pub trait BlockTestCase: Sized {
    fn parse_n(reader: &mut impl BufRead, n: usize) -> Result<Self, Error>;
}

impl<T: InlineTestCase> BlockTestCase for Vec<T> {
    fn parse_n(reader: &mut impl BufRead, n: usize) -> Result<Self, Error> {
        let mut buffer = String::new();

        (0..n)
            .map(|_| {
                buffer.clear();
                reader.read_line(&mut buffer)?;
                T::parse_line(&buffer)
            })
            .collect()
    }
}

// ---
// --- TestCase
// ---

/// Trait for a testcase that can consume freely from its input.
pub trait TestCase: Sized {
    fn parse(reader: &mut impl BufRead) -> Result<Self, Error>;
}

impl<T: InlineTestCase> TestCase for T {
    fn parse(reader: &mut impl BufRead) -> Result<Self, Error> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        T::parse_line(&line)
    }
}
