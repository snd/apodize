# apodize

[![Build Status][build-image]][build-link]
[![Crate][crate-img]][crate-link]
[![Docs][docs-img]][docs-link]
[![License][license-image]][license-link]

Very simple [rust](https://www.rust-lang.org/)
iterators that yield
[generalized cosine](https://snd.github.io/apodize/apodize/fn.cosine_iter.html),
[hanning](https://snd.github.io/apodize/apodize/fn.hanning_iter.html),
[hamming](https://snd.github.io/apodize/apodize/fn.hamming_iter.html),
[blackman](https://snd.github.io/apodize/apodize/fn.blackman_iter.html),
[nuttall](https://snd.github.io/apodize/apodize/fn.nuttall_iter.html)
and
[triangular](https://snd.github.io/apodize/apodize/fn.triangular_iter.html)
windows

Useful for
smoothing the sharp discontinuities at the beginning and end
of a slice of samples when doing a
[short time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform).
windowing also improves temporal resolution by making
the signal near the time
being analyzed have higher weight than the signal
further away from the time being analyzed.

[Documentation and Example][docs-link]

[Read before contributing](contributing.md)

Licensed under either [apache-2.0](LICENSE-APACHE) ([tl;dr](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0))) or [MIT](LICENSE-MIT) ([tl;dr](https://tldrlegal.com/license/mit-license)) at your option

[build-image]: https://secure.travis-ci.org/snd/apodize.svg?branch=master
[build-link]: https://travis-ci.org/snd/apodize

[crate-img]: https://img.shields.io/crates/v/apodize.svg
[crate-link]: https://crates.io/crates/apodize

[docs-img]: https://docs.rs/apodize/badge.svg
[docs-link]: https://docs.rs/apodize

[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/snd/apodize/blob/master/LICENSE-MIT
