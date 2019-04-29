/*!

**very simple rust iterators that yield
[generalized cosine](https://snd.github.io/apodize/apodize/fn.cosine_iter.html),
[hanning](https://snd.github.io/apodize/apodize/fn.hanning_iter.html),
[hamming](https://snd.github.io/apodize/apodize/fn.hamming_iter.html),
[blackman](https://snd.github.io/apodize/apodize/fn.blackman_iter.html),
[nuttall](https://snd.github.io/apodize/apodize/fn.nuttall_iter.html)
and
[triangular](https://snd.github.io/apodize/apodize/fn.triangular_iter.html)
windows**

useful for
smoothing the sharp discontinuities at the beginning and end
of a slice of samples when doing a
[short time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).
windowing also improves temporal resolution by making
the signal near the time
being analyzed have higher weight than the signal
further away from the time being analyzed.

all iterators yield `f64`s.
previously they were generic over the floating type used.
this was removed because it introduced complexity.
if you need `f32`s just `.map(|x| x as f32)` over the iterator.

## example

you will most likely want to collect the yielded values
in a vector and then multiply that window vector repeatedly with some
data vectors to apodize them.

here is an example of that for a hanning window (hamming, blackman and nuttall are analogous).

```
#[macro_use]
extern crate approx;

fn main() {
    // create a hanning window iterator of size 7
    // and collect the values it yields in a Vec
    let window = apodize::hanning_iter(7).collect::<Vec<f64>>();

    let expected = vec![
        0.0,
        0.24999999999999994,
        0.7499999999999999,
        1.0,
        0.7500000000000002,
        0.25,
        0.0
    ];
    assert_ulps_eq!(window.as_slice(), expected.as_slice(), max_ulps = 10);

    // some data we want to apodize (multiply with the window)
    let data: Vec<f64> = vec![1., 2., 3., 4., 5., 6., 7.];

    // buffer that will hold data * window
    let mut windowed_data = vec![0.; data.len()];

    for (windowed, (window, data)) in windowed_data.iter_mut().zip(window.iter().zip(data.iter())) {
        *windowed = *window * *data;
    }

    let expected = vec![
        0.0,
        0.4999999999999999,
        2.2499999999999996,
        4.0,
        3.750000000000001,
        1.5,
        0.0
    ];
    assert_ulps_eq!(windowed_data.as_slice(), expected.as_slice(), max_ulps = 10);
}
```
*/

use std::f64::consts::PI;

/// holds the window coefficients and
/// iteration state of an iterator for a cosine window
#[derive(Clone, Debug)]
pub struct CosineWindowIter {
    /// coefficient `a` of the cosine window
    pub a: f64,
    /// coefficient `b` of the cosine window
    pub b: f64,
    /// coefficient `c` of the cosine window
    pub c: f64,
    /// coefficient `d` of the cosine window
    pub d: f64,
    /// the current `index` of the iterator
    pub index: usize,
    /// `size` of the cosine window
    pub size: usize,
}

impl Iterator for CosineWindowIter {
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        if self.index == self.size {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(cosine_at(self.a, self.b, self.c, self.d, self.size, index))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for CosineWindowIter {
    #[inline]
    fn len(&self) -> usize {
        self.size
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
pub fn cosine_at(a: f64, b: f64, c: f64, d: f64, size: usize, index: usize) -> f64 {
    let x = (PI * index as f64) / (size - 1) as f64;
    let b_ = b * (2. * x).cos();
    let c_ = c * (4. * x).cos();
    let d_ = d * (6. * x).cos();
    (a - b_) + (c_ - d_)
}

/// returns an iterator that yields the values for a [cosine
/// window](https://en.wikipedia.org/wiki/Window_function#Hann_.28Hanning.29_window) of `size`
/// with the coefficients `a`, `b`, `c` and `d`
/// # Panics
/// panics unless `1 < size`
pub fn cosine_iter(a: f64, b: f64, c: f64, d: f64, size: usize) -> CosineWindowIter {
    assert!(1 < size);
    CosineWindowIter {
        a,
        b,
        c,
        d,
        index: 0,
        size,
    }
}

/// returns an iterator that yields the values for a [hanning
/// window](https://en.wikipedia.org/wiki/Window_function#Hann_.28Hanning.29_window) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn hanning_iter(size: usize) -> CosineWindowIter {
    cosine_iter(0.5, 0.5, 0., 0., size)
}

/// returns an iterator that yields the values for a [hamming
/// window](https://en.wikipedia.org/wiki/Window_function#Hamming_window) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn hamming_iter(size: usize) -> CosineWindowIter {
    cosine_iter(0.54, 0.46, 0., 0., size)
}

/// returns an iterator that yields the values for a [blackman
/// window](https://en.wikipedia.org/wiki/Window_function#Blackman_windows) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn blackman_iter(size: usize) -> CosineWindowIter {
    cosine_iter(0.35875, 0.48829, 0.14128, 0.01168, size)
}

/// returns an iterator that yields the values for a [nuttall
/// window](https://en.wikipedia.org/wiki/Window_function#Nuttall_window.2C_continuous_first_derivative) of `size`
/// # Panics
/// panics unless `1 < size`
pub fn nuttall_iter(size: usize) -> CosineWindowIter {
    cosine_iter(0.355_768, 0.487_396, 0.144_232, 0.012_604, size)
}

/// holds the iteration state of an iterator for a triangular window
#[derive(Clone, Debug)]
pub struct TriangularWindowIter {
    pub l: usize,
    /// the current `index` of the iterator
    pub index: usize,
    /// `size` of the triangular window
    pub size: usize,
}

/// returns the value of the
/// [triangular window]
/// (https://en.wikipedia.org/wiki/Window_function#Triangular_window)
/// of `size`
/// at `index`
#[inline]
pub fn triangular_at(l: usize, size: usize, index: usize) -> f64 {
    // ends with zeros if l == size - 1
    // if l == size - 1 && index == 0 then 1 - 1 / 1 == 0
    // if l == size - 1 && index == size - 1 then 1 - 0 / 1 == 0
    1. - ((index as f64 - (size - 1) as f64 / 2.) / (l as f64 / 2.)).abs()
}

impl Iterator for TriangularWindowIter {
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        if self.index == self.size {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(triangular_at(self.l, self.size, index))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for TriangularWindowIter {
    #[inline]
    fn len(&self) -> usize {
        self.size
    }
}

/// returns an iterator that yields the values for a [triangular
/// window](https://en.wikipedia.org/wiki/Window_function#Triangular_window)
/// if `l = size - 1` then the outermost values of the window are `0`.
/// if `l = size` then the outermost values of the window are higher.
/// if `l = size + 1` then the outermost values of the window are even higher.
/// # Panics
/// panics unless `0 < size`
pub fn triangular_iter(size: usize) -> TriangularWindowIter {
    assert!(0 < size);
    TriangularWindowIter {
        l: size,
        size,
        index: 0,
    }
}
