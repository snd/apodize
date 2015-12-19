use std::f64::consts::PI;

pub type Float = f64;

pub struct CosineWindowIter {
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    index: usize,
    size: usize,
}

impl CosineWindowIter {
    /// the value of the cosine window of size `size`
    /// with the coefficients `a`, `b`, `c` and `d`
    /// at index `index`
    /// TODO find better name
    #[inline]
    pub fn value_at_index(a: Float,
                          b: Float,
                          c: Float,
                          d: Float,
                          size: usize,
                          index: usize)
                          -> Float {
        let x = (PI * index as f64) / (size - 1) as f64;
        let b_ = b * (2. * x).cos();
        let c_ = c * (4. * x).cos();
        let d_ = d * (6. * x).cos();
        (a - b_) + (c_ - d_)
    }
}

impl Iterator for CosineWindowIter {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        if self.index == self.size {
            return None;
        }
        let index = self.index;
        self.index += 1;
        Some(CosineWindowIter::value_at_index(self.a,
                                              self.b,
                                              self.c,
                                              self.d,
                                              self.size,
                                              index))
    }
}

/// https://en.wikipedia.org/wiki/Window_function#Cosine_window
pub fn cosine(a: Float,
              b: Float,
              c: Float,
              d: Float,
              size: usize)
              -> CosineWindowIter {
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

pub fn hanning(size: usize) -> CosineWindowIter {
    cosine(0.5, 0.5, 0., 0., size)
}

pub fn hamming(size: usize) -> CosineWindowIter {
    cosine(0.54, 0.46, 0., 0., size)
}

pub fn blackman(size: usize) -> CosineWindowIter {
    cosine(0.35875, 0.48829, 0.14128, 0.01168, size)
}

pub fn nuttall(size: usize) -> CosineWindowIter {
    cosine(0.355768, 0.487396, 0.144232, 0.012604, size)
}
