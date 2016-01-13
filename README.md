# apodize

*status: working. tested. api still in flux.*

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
of each slice of samples when doing a
[short time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).
windowing also improves temporal resolution by making
the signal near the time
being analyzed have higher weight than the signal
further away from the time being analyzed.

to use add `apodize = "*"`
to the `[dependencies]` section of your `Cargo.toml` and call `extern crate apodize;` in your code.

## [read the documentation for an example and more !](https://snd.github.io/apodize/apodize/index.html)

### [contributing](contributing.md)

### licensed under either of [apache-2.0](LICENSE-APACHE) ([tl;dr](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0))) or [MIT](LICENSE-MIT) ([tl;dr](https://tldrlegal.com/license/mit-license)) at your option
