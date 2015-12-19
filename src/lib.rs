//! this is the module documentation

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

/// the value of the cosine window of size `size`
/// with the coefficients `a`, `b`, `c` and `d`
/// at index `index`
/// TODO find better name
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

/// https://en.wikipedia.org/wiki/Window_function#Cosine_window
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

pub fn hanning<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.5).unwrap(), T::from(0.5).unwrap(), T::from(0.).unwrap(), T::from(0.).unwrap(), size)
}

pub fn hamming<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.54).unwrap(), T::from(0.46).unwrap(), T::from(0.).unwrap(), T::from(0.).unwrap(), size)
}

pub fn blackman<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.35875).unwrap(), T::from(0.48829).unwrap(), T::from(0.14128).unwrap(), T::from(0.01168).unwrap(), size)
}

pub fn nuttall<T: Float + Pi>(size: usize) -> CosineWindowIter<T> {
    cosine::<T>(T::from(0.355768).unwrap(), T::from(0.487396).unwrap(), T::from(0.144232).unwrap(), T::from(0.012604).unwrap(), size)
}
