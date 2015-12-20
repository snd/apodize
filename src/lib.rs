/*!

```
use std::ops::Mul;

#[macro_use]
extern crate nalgebra;
use nalgebra::{ApproxEq, DVec};

extern crate apodize;
use apodize::{hanning};

fn main() {
    let data: DVec<f64> = vec![1., 2., 3., 4., 5., 6., 7.].iter().map(|x| *x).collect();

    let size = 7;
    let window = hanning::<f64>(size).collect::<DVec<f64>>();

    assert_approx_eq_ulps!(
        vec![
            0.0,
            0.24999999999999994,
            0.7499999999999999,
            1.0,
            0.7500000000000002,
            0.25,
            0.0].iter().map(|x| *x).collect(),
        window,
        10);

    // apply window to data
    let windowed_data = window.mul(data);

    assert_approx_eq_ulps!(
        vec![
            0.0,
            0.4999999999999999,
            2.2499999999999996,
            4.0,
            3.750000000000001,
            1.5,
            0.0].iter().map(|x| *x).collect(),
        windowed_data,
        10);
}
```
*/
extern crate num;
use num::traits::Float;

pub mod pi;
use pi::Pi;

pub struct CosineWindowIter<T> {
    a: T,
    b: T,
    c: T,
    d: T,
    index: usize,
    size: usize,
}

impl<T: Float + Pi> Iterator for CosineWindowIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.size {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(cosine_window_value_at_index(self.a,
                                          self.b,
                                          self.c,
                                          self.d,
                                          self.size,
                                          index))
    }
}

/// returns the value of the [cosine
/// window](https://en.wikipedia.org/wiki/Window_function#Higher-order_generalized_cosine_windows)
/// with the coefficients `a`, `b`, `c` and `d`, of `size`
/// at index `index`
#[inline]
pub fn cosine_window_value_at_index<T: Float + Pi>(a: T,
                                                   b: T,
                                                   c: T,
                                                   d: T,
                                                   size: usize,
                                                   index: usize)
    -> T {
        let pi: T = T::pi();
        let x: T = (pi * T::from(index).unwrap()) / T::from(size - 1).unwrap();
        let b_ = b * (T::from(2.).unwrap() * x).cos();
        let c_ = c * (T::from(4.).unwrap() * x).cos();
        let d_ = d * (T::from(6.).unwrap() * x).cos();
        ((a - b_) + (c_ - d_)).max(T::from(0.).unwrap())
    }

/// returns an iterator that yields the values for a [cosine
/// window](https://en.wikipedia.org/wiki/Window_function#Hann_.28Hanning.29_window) of `size`
/// with four coefficients `a`, `b`, `c` and `d`
pub fn cosine<T: Float + Pi>(a: T,
                             b: T,
                             c: T,
                             d: T,
                             size: usize)
    -> CosineWindowIter<T> {
        assert!(size > 1);
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
pub fn hanning<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.5).unwrap(), T::from(0.5).unwrap(), T::from(0.).unwrap(), T::from(0.).unwrap(), size)
}

/// returns an iterator that yields the values for a [hamming
/// window](https://en.wikipedia.org/wiki/Window_function#Hamming_window) of `size`
pub fn hamming<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.54).unwrap(), T::from(0.46).unwrap(), T::from(0.).unwrap(), T::from(0.).unwrap(), size)
}

/// returns an iterator that yields the values for a [blackman
/// window](https://en.wikipedia.org/wiki/Window_function#Blackman_windows) of `size`
pub fn blackman<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.35875).unwrap(), T::from(0.48829).unwrap(), T::from(0.14128).unwrap(), T::from(0.01168).unwrap(), size)
}

/// returns an iterator that yields the values for a [nuttall
/// window](https://en.wikipedia.org/wiki/Window_function#Nuttall_window.2C_continuous_first_derivative) of `size`
pub fn nuttall<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.355768).unwrap(), T::from(0.487396).unwrap(), T::from(0.144232).unwrap(), T::from(0.012604).unwrap(), size)
}
