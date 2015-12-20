# apodize

[![](https://meritbadge.herokuapp.com/cosine-window)](https://crates.io/crates/cosine-window)
[![Build Status](https://travis-ci.org/snd/cosine-window.svg?branch=master)](https://travis-ci.org/snd/cosine-window/branches)

**iterators that yield generalized cosine, hanning, hamming, blackman and nuttall windows**

```rust
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

### [contributing](contributing.md)

**bugfixes, issues and discussion are always welcome.  
kindly [ask](https://github.com/snd/window/issues/new) before implementing new features.**

i will happily merge pull requests that fix bugs with reasonable code.

i will only merge pull requests that modify/add functionality
if the changes align with my goals for this package,
are well written, documented and tested.

**communicate !**  
[write an issue](https://github.com/snd/window/issues/new) to start a discussion before writing code that may or may not get merged.

[this project adheres to the contributor covenant 1.2](CODE_OF_CONDUCT.MD). by participating, you are expected to uphold this code. please report unacceptable behavior to kruemaxi@gmail.com.

## [license: MIT](LICENSE)
