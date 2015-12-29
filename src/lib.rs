/*!

**rust iterators that yield
[generalized cosine](https://snd.github.io/apodize/apodize/fn.cosine_iter.html),
[hanning](https://snd.github.io/apodize/apodize/fn.hanning_iter.html),
[hamming](https://snd.github.io/apodize/apodize/fn.hamming_iter.html),
[blackman](https://snd.github.io/apodize/apodize/fn.blackman_iter.html),
and
[nuttall](https://snd.github.io/apodize/apodize/fn.nuttall_iter.html)
windows**

useful for
smoothing the sharp discontinuities at the edges (beginning and end)
of each slice of samples when doing a
[short time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).
windowing also improves temporal resolution by making
the signal near the time
being analyzed have higher weight than the signal
further away from the time being analyzed.

to use add `apodize = "0.1.1"`
to the `[dependencies]` section of your `Cargo.toml` and `extern crate apodize;` in your code.

you will most likely want to collect the yielded values
in a vector and then multiply that window vector repeatedly with some
data vectors to apodize them.

here is an example of that for a hanning window (hamming, blackman and nuttall are analogous):

```
use std::ops::Mul;

#[macro_use]
extern crate nalgebra;
use nalgebra::{ApproxEq, DVec};

#[macro_use]
extern crate apodize;
use apodize::{hanning_iter};

fn main() {
    // create a hanning window iterator of size 7
    // and collect the values it yields in an nalgebra::DVec.
    // hanning_iter is generic over the type
    // of floating point number yielded (f32 or f64).
    // we use f64 here.
    let window = hanning_iter::<f64>(7).collect::<DVec<f64>>();

    assert_approx_eq_ulps!(
        window,
        dvec![
            0.0,
            0.24999999999999994,
            0.7499999999999999,
            1.0,
            0.7500000000000002,
            0.25,
            0.0],
        10);

    // some data we want to window
    let data: DVec<f64> = dvec![1., 2., 3., 4., 5., 6., 7.];

    // apply window to data
    let windowed_data = window.mul(data);

    assert_approx_eq_ulps!(
        windowed_data,
        dvec![
            0.0,
            0.4999999999999999,
            2.2499999999999996,
            4.0,
            3.750000000000001,
            1.5,
            0.0],
        10);
}
```
*/

extern crate num;
use num::traits::Float;

/// helper shorthand macro for shorter, more readable code:
/// `from!(T, x)` -> `T::from(x).unwrap()`
#[macro_export]
macro_rules! from {
    ($typ:ty, $val:expr) => { <$typ>::from($val).unwrap() }
}

/// build an `nalgebra::DVec` as easy as a `std::Vec`.
/// for shorter, more readable code in tests and examples.
#[macro_export]
macro_rules! dvec {
    ($( $x:expr ),*) => { DVec {at: vec![$($x),*]} };
    ($($x:expr,)*) => { dvec![$($x),*] };
}

/// the constant pi for generic floating point types.
/// workaround until [associated
/// constants](https://doc.rust-lang.org/book/associated-constants.html)
/// are stable.
pub trait CanRepresentPi {
    fn pi() -> Self;
}

impl CanRepresentPi for f32 {
    #[inline]
    fn pi() -> Self { std::f32::consts::PI }
}

impl CanRepresentPi for f64 {
    #[inline]
    fn pi() -> Self { std::f64::consts::PI }
}

/// holds the window coefficients and
/// iteration state of a cosine window iterator
pub struct CosineWindowIter<T> {
    /// coefficient `a` of the cosine window
    pub a: T,
    /// coefficient `b` of the cosine window
    pub b: T,
    /// coefficient `c` of the cosine window
    pub c: T,
    /// coefficient `d` of the cosine window
    pub d: T,
    /// the current index of the iterator
    pub index: usize,
    /// `size` of the cosine window
    pub size: usize,
}

impl<T: Float + CanRepresentPi> ExactSizeIterator for CosineWindowIter<T> {
    fn len(&self) -> usize { self.size }
}

impl<T: Float + CanRepresentPi> Iterator for CosineWindowIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.size {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(cosine_at(self.a,
                       self.b,
                       self.c,
                       self.d,
                       self.size,
                       index))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.index;
        (remaining, Some(remaining))
    }
}

/// returns the value of the [cosine
/// window](https://en.wikipedia.org/wiki/Window_function#Higher-order_generalized_cosine_windows)
/// of `size`
/// with the coefficients `a`, `b`, `c` and `d`
/// at `index`
/// # Panics
/// panics unless `1 < size`
#[inline]
pub fn cosine_at<T: Float + CanRepresentPi>(
    a: T,
    b: T,
    c: T,
    d: T,
    size: usize,
    index: usize)
    -> T {
        let pi: T = T::pi();
        let x: T = (pi * from!(T, index)) / from!(T, size - 1);
        let b_ = b * (from!(T, 2.) * x).cos();
        let c_ = c * (from!(T, 4.) * x).cos();
        let d_ = d * (from!(T, 6.) * x).cos();
        (a - b_) + (c_ - d_)
    }

/// returns an iterator that yields the values for a [cosine
/// window](https://en.wikipedia.org/wiki/Window_function#Hann_.28Hanning.29_window) of `size`
/// with the coefficients `a`, `b`, `c` and `d`
/// # Panics
/// panics unless `1 < size`
pub fn cosine_iter<T: Float + CanRepresentPi>(
    a: T,
    b: T,
    c: T,
    d: T,
    size: usize)
    -> CosineWindowIter<T> {
        assert!(1 < size);
        CosineWindowIter {
            a: a,
            b: b,
            c: c,
            d: d,
            index: 0,
            size: size,
        }
    }

/// returns an iterator that yields the values for a [hanning
/// window](https://en.wikipedia.org/wiki/Window_function#Hann_.28Hanning.29_window) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn hanning_iter<T: Float + CanRepresentPi>(size: usize) -> CosineWindowIter<T> {
    cosine_iter::<T>(
        from!(T, 0.5),
        from!(T, 0.5),
        from!(T, 0.),
        from!(T, 0.),
        size)
}

/// returns an iterator that yields the values for a [hamming
/// window](https://en.wikipedia.org/wiki/Window_function#Hamming_window) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn hamming_iter<T: Float + CanRepresentPi>(size: usize) -> CosineWindowIter<T> {
    cosine_iter::<T>(
        from!(T, 0.54),
        from!(T, 0.46),
        from!(T, 0.),
        from!(T, 0.),
        size)
}

/// returns an iterator that yields the values for a [blackman
/// window](https://en.wikipedia.org/wiki/Window_function#Blackman_windows) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn blackman_iter<T: Float + CanRepresentPi>(size: usize) -> CosineWindowIter<T> {
    cosine_iter::<T>(
        from!(T, 0.35875),
        from!(T, 0.48829),
        from!(T, 0.14128),
        from!(T, 0.01168),
        size)
}

/// returns an iterator that yields the values for a [nuttall
/// window](https://en.wikipedia.org/wiki/Window_function#Nuttall_window.2C_continuous_first_derivative) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn nuttall_iter<T: Float + CanRepresentPi>(size: usize) -> CosineWindowIter<T> {
    cosine_iter::<T>(
        from!(T, 0.355768),
        from!(T, 0.487396),
        from!(T, 0.144232),
        from!(T, 0.012604),
        size)
}
