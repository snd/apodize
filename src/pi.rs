//! workaround until [associated
//! constants](https://doc.rust-lang.org/book/associated-constants.html)
//! are stable.

use std;

pub trait Pi {
    fn pi() -> Self;
}

impl Pi for f32 {
    #[inline]
    fn pi() -> Self { std::f32::consts::PI }
}

impl Pi for f64 {
    #[inline]
    fn pi() -> Self { std::f64::consts::PI }
}
