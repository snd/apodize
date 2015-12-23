#![feature(test)]

extern crate test;

extern crate apodize;
use apodize::CanRepresentPi;

#[bench]
fn bench_cosine_at_f32(b: &mut test::Bencher) {
    b.iter(|| {
        apodize::cosine_at::<f32>(
            0.355768,
            0.487396,
            0.144232,
            0.012604,
            1024,
            400
        )
    });
}

#[bench]
fn bench_cosine_at_f64(b: &mut test::Bencher) {
    b.iter(|| {
        apodize::cosine_at::<f64>(
            0.355768,
            0.487396,
            0.144232,
            0.012604,
            1024,
            400
        )
    });
}

#[bench]
fn bench_hanning_iter_1024_f32(b: &mut test::Bencher) {
    b.iter(|| {
        apodize::hanning_iter(1024).collect::<Vec<f32>>()
    });
}

#[bench]
fn bench_hanning_iter_1024_f64(b: &mut test::Bencher) {
    b.iter(|| {
        apodize::hanning_iter(1024).collect::<Vec<f64>>()
    });
}

// this should take 0 ns/iter
#[bench]
fn bench_pi_f32_function_call_optimized_away(b: &mut test::Bencher) {
    b.iter(|| { f32::pi() });
}

// this should take 0 ns/iter
#[bench]
fn bench_pi_f64_function_call_optimized_away(b: &mut test::Bencher) {
    b.iter(|| { f64::pi() });
}
