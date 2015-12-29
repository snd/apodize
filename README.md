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

### [read on in the generated documentation](https://snd.github.io/apodize/apodize/index.html)

### [contributing](contributing.md)

### [license: MIT](LICENSE)
