# apodize

[![Build Status](https://travis-ci.org/snd/apodize.svg?branch=master)](https://travis-ci.org/snd/apodize/branches)
[![](https://meritbadge.herokuapp.com/apodize)](https://crates.io/crates/apodize)

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
of each segment when doing a
[short time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).
windowing also improves temporal resolution by making
the signal near the time
being analyzed have higher weight than the signal
further away from the time being analyzed.

to use add `apodize = "0.1.1"`
to the `[dependencies]` section of your `Cargo.toml` and `extern crate apodize;` in your code.

### [generated documentation](https://snd.github.io/apodize/apodize/index.html)

you will most likely want to collect the yielded values
in a vector and then multiply that window vector repeatedly with some
data vectors to apodize them.

here is an example of that for a hanning window (hamming, blackman and nuttall are analogous):
```rust
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

### [contributing](contributing.md)

### [license: MIT](LICENSE)
