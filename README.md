# apodize

[![](https://meritbadge.herokuapp.com/cosine-window)](https://crates.io/crates/cosine-window)
[![Build Status](https://travis-ci.org/snd/cosine-window.svg?branch=master)](https://travis-ci.org/snd/cosine-window/branches)

**iterators that yield generalized cosine, hanning, hamming, blackman and nuttall windows**

```rust
extern crate apodize;
use apodize::{hanning, cosine};

fn main() {
  let size = 10;
  let hanning_window_iter = hanning(size);
  // store window in a vector
  let hanning_window_vec = hanning_window_iter.collect::<Vec<f64>>();
  assert(

  let nuttall_window_iter = cosine(
    0.355768, 0.487396, 0.144232, 0.012604, size);
  // store nutall window in a vector
  let nuttall_window_vec = nuttall_window_iter.collect::<Vec<f64>>();
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
